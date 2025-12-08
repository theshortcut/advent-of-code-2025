advent_of_code::solution!(8);

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse junction network: empty or invalid input")
    }
}

impl std::error::Error for ParseError {}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    num_components: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            num_components: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }

        self.num_components -= 1;
        true
    }

    #[inline]
    fn component_count(&self) -> usize {
        self.num_components
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = vec![0; n];

        for i in 0..n {
            let root = self.find(i);
            sizes[root] += 1;
        }

        sizes.into_iter().filter(|&s| s > 0).collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    #[inline]
    fn distance_squared(&self, other: &Point3D) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

struct JunctionNetwork {
    boxes: Vec<Point3D>,
    edges: Vec<(i64, usize, usize)>,
}

impl TryFrom<&str> for JunctionNetwork {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let boxes: Vec<Point3D> = input
            .lines()
            .filter_map(|line| {
                let parts: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
                if parts.len() == 3 {
                    Some(Point3D {
                        x: parts[0],
                        y: parts[1],
                        z: parts[2],
                    })
                } else {
                    None
                }
            })
            .collect();

        if boxes.is_empty() {
            return Err(ParseError);
        }

        let n = boxes.len();
        let mut edges = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..n {
            for j in i + 1..n {
                let dist_sq = boxes[i].distance_squared(&boxes[j]);
                edges.push((dist_sq, i, j));
            }
        }

        edges.sort_unstable_by_key(|&(dist, _, _)| dist);

        Ok(JunctionNetwork { boxes, edges })
    }
}

impl JunctionNetwork {

    fn connect_k_closest(&self, k: usize) -> Option<u64> {
        let mut uf = UnionFind::new(self.boxes.len());
        let connections = k.min(self.edges.len());

        for i in 0..connections {
            let (_, u, v) = self.edges[i];
            uf.union(u, v);
        }

        let mut sizes = uf.get_component_sizes();
        sizes.sort_unstable_by(|a, b| b.cmp(a));

        if sizes.len() >= 3 {
            Some((sizes[0] * sizes[1] * sizes[2]) as u64)
        } else {
            None
        }
    }

    fn connect_until_single_circuit(&self) -> Option<u64> {
        let mut uf = UnionFind::new(self.boxes.len());

        for &(_, u, v) in &self.edges {
            uf.union(u, v);

            if uf.component_count() == 1 {
                return Some((self.boxes[u].x as u64) * (self.boxes[v].x as u64));
            }
        }

        None
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let network = JunctionNetwork::try_from(input).ok()?;
    network.connect_k_closest(1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let network = JunctionNetwork::try_from(input).ok()?;
    network.connect_until_single_circuit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let network = JunctionNetwork::try_from(
            advent_of_code::template::read_file("examples", DAY).as_str(),
        )
        .unwrap();
        let result = network.connect_k_closest(10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
