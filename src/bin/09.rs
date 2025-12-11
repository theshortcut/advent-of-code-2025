advent_of_code::solution!(9);

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse tile coordinates: invalid input")
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

struct TileGrid {
    tiles: Vec<Point>,
    valid_ranges: Vec<Vec<(i32, i32)>>,
    min_y: i32,
    max_y: i32,
}

impl TryFrom<&str> for TileGrid {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let tiles: Vec<Point> = input
            .lines()
            .filter(|line| !line.is_empty())
            .filter_map(|line| {
                let mut parts = line.split(',');
                let x = parts.next()?.parse().ok()?;
                let y = parts.next()?.parse().ok()?;
                Some(Point { x, y })
            })
            .collect();

        if tiles.is_empty() {
            return Err(ParseError);
        }

        let min_y = tiles.iter().map(|p| p.y).min().unwrap();
        let max_y = tiles.iter().map(|p| p.y).max().unwrap();

        let (valid_ranges, min_y_actual, max_y_actual) =
            Self::compute_valid_ranges(&tiles, min_y, max_y);

        Ok(TileGrid {
            tiles,
            valid_ranges,
            min_y: min_y_actual,
            max_y: max_y_actual,
        })
    }
}

impl TileGrid {
    /// Compute valid x-ranges for each y-coordinate using a scanline algorithm
    ///
    /// For each horizontal line y:
    /// 1. Find all vertical edges that cross this line
    /// 2. Sort crossing points and pair them to create valid ranges
    /// 3. Include any horizontal edges at this y-coordinate
    /// 4. Merge overlapping ranges
    ///
    /// Returns (ranges_vec, min_y, max_y) where ranges_vec[i] corresponds to y = min_y + i
    fn compute_valid_ranges(
        tiles: &[Point],
        min_y: i32,
        max_y: i32,
    ) -> (Vec<Vec<(i32, i32)>>, i32, i32) {
        let n = tiles.len();
        let height = (max_y - min_y + 1) as usize;
        let mut ranges = vec![Vec::new(); height];

        for y in min_y..=max_y {
            let y_idx = (y - min_y) as usize;
            let mut crossings = Vec::new();

            for i in 0..n {
                let p1 = tiles[i];
                let p2 = tiles[(i + 1) % n];

                // Check for vertical edges crossing this horizontal line
                if p1.x == p2.x && ((p1.y <= y && y < p2.y) || (p2.y <= y && y < p1.y)) {
                    crossings.push(p1.x);
                }

                // Check for horizontal edges at this y-coordinate
                if p1.y == p2.y && p1.y == y {
                    let x_min = p1.x.min(p2.x);
                    let x_max = p1.x.max(p2.x);
                    ranges[y_idx].push((x_min, x_max));
                }
            }

            // Create ranges from vertical edge crossings (inside the polygon)
            crossings.sort_unstable();
            for chunk in crossings.chunks(2) {
                if chunk.len() == 2 {
                    ranges[y_idx].push((chunk[0], chunk[1]));
                }
            }

            // Merge overlapping ranges for this y-coordinate
            Self::merge_ranges(&mut ranges[y_idx]);
        }

        (ranges, min_y, max_y)
    }

    /// Merge overlapping or adjacent ranges in-place
    fn merge_ranges(ranges: &mut Vec<(i32, i32)>) {
        if ranges.is_empty() {
            return;
        }

        ranges.sort_unstable();
        let mut merged: Vec<(i32, i32)> = Vec::new();

        for &(start, end) in ranges.iter() {
            if let Some(last) = merged.last_mut() {
                if start <= last.1 {
                    // Overlapping or adjacent - extend the last range
                    last.1 = last.1.max(end);
                } else {
                    // Non-overlapping - add new range
                    merged.push((start, end));
                }
            } else {
                merged.push((start, end));
            }
        }

        *ranges = merged;
    }

    /// Find the largest rectangle using any two red tiles as opposite corners
    fn largest_rectangle_area(&self) -> u64 {
        let n = self.tiles.len();
        let mut max_area = 0u64;

        for i in 0..n {
            for j in i + 1..n {
                let p1 = self.tiles[i];
                let p2 = self.tiles[j];

                let width = (p1.x - p2.x).unsigned_abs() as u64 + 1;
                let height = (p1.y - p2.y).unsigned_abs() as u64 + 1;
                let area = width * height;

                max_area = max_area.max(area);
            }
        }

        max_area
    }

    /// Check if the x-range [x1, x2] is entirely within valid ranges for y
    #[inline]
    fn is_x_range_valid(&self, y: i32, x1: i32, x2: i32) -> bool {
        if y < self.min_y || y > self.max_y {
            return false;
        }

        let y_idx = (y - self.min_y) as usize;
        let ranges = &self.valid_ranges[y_idx];

        ranges
            .iter()
            .any(|&(range_min, range_max)| range_min <= x1 && x2 <= range_max)
    }

    /// Check if a rectangle is valid (all tiles are red or green)
    #[inline]
    fn is_valid_rectangle(&self, p1: Point, p2: Point) -> bool {
        let min_x = p1.x.min(p2.x);
        let max_x = p1.x.max(p2.x);
        let min_y = p1.y.min(p2.y);
        let max_y = p1.y.max(p2.y);

        // Check if every horizontal slice of the rectangle is valid
        for y in min_y..=max_y {
            if !self.is_x_range_valid(y, min_x, max_x) {
                return false;
            }
        }
        true
    }

    /// Find the largest valid rectangle (all tiles red or green)
    fn largest_valid_rectangle_area(&self) -> u64 {
        let n = self.tiles.len();

        // Pre-compute all candidate areas with their indices
        let mut candidates: Vec<(u64, usize, usize)> = (0..n)
            .flat_map(|i| {
                (i + 1..n).map(move |j| {
                    let p1 = self.tiles[i];
                    let p2 = self.tiles[j];
                    let width = (p1.x - p2.x).unsigned_abs() as u64 + 1;
                    let height = (p1.y - p2.y).unsigned_abs() as u64 + 1;
                    let area = width * height;
                    (area, i, j)
                })
            })
            .collect();

        // Sort by area descending
        candidates.sort_unstable_by_key(|&(area, _, _)| std::cmp::Reverse(area));

        // Check candidates in order of decreasing area
        for &(area, i, j) in &candidates {
            let p1 = self.tiles[i];
            let p2 = self.tiles[j];

            if self.is_valid_rectangle(p1, p2) {
                return area;
            }
        }

        0
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = TileGrid::try_from(input).ok()?;
    Some(grid.largest_rectangle_area())
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = TileGrid::try_from(input).ok()?;
    Some(grid.largest_valid_rectangle_area())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
