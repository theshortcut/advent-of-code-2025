advent_of_code::solution!(12);

type Coord = (i32, i32);
type Shape = Vec<Coord>;

/// Represents a polyomino present shape
#[derive(Debug, Clone)]
struct Present {
    cells: Vec<Coord>,
}

impl Present {
    fn from_lines(lines: &[&str]) -> Self {
        let mut cells = Vec::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    cells.push((x as i32, y as i32));
                }
            }
        }
        Present { cells }
    }

    fn rotations_and_flips(&self) -> Vec<Shape> {
        let transformations: [fn(Coord) -> Coord; 8] = [
            |(x, y)| (x, y),
            |(x, y)| (-y, x),
            |(x, y)| (-x, -y),
            |(x, y)| (y, -x),
            |(x, y)| (-x, y),
            |(x, y)| (y, x),
            |(x, y)| (x, -y),
            |(x, y)| (-y, -x),
        ];

        let mut unique_variants = Vec::with_capacity(8);

        for &transform in &transformations {
            let transformed: Shape = self.cells.iter().map(|&c| transform(c)).collect();
            let normalized = normalize(&transformed);

            if !unique_variants.contains(&normalized) {
                unique_variants.push(normalized);
            }
        }

        unique_variants
    }
}

#[inline]
fn normalize(coords: &[Coord]) -> Shape {
    if coords.is_empty() {
        return vec![];
    }

    let min_x = coords.iter().map(|&(x, _)| x).min().unwrap();
    let min_y = coords.iter().map(|&(_, y)| y).min().unwrap();

    let mut normalized: Shape = coords
        .iter()
        .map(|&(x, y)| (x - min_x, y - min_y))
        .collect();
    normalized.sort_unstable();
    normalized
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;
    let mut shapes = Vec::new();

    while i < lines.len() {
        if lines[i].contains(':') && !lines[i].contains('x') {
            i += 1;
            let mut shape_lines = Vec::new();
            while i < lines.len() && !lines[i].is_empty() && !lines[i].contains(':') {
                shape_lines.push(lines[i]);
                i += 1;
            }
            if !shape_lines.is_empty() {
                shapes.push(Present::from_lines(&shape_lines));
            }
        } else {
            i += 1;
        }
    }

    let mut regions = Vec::new();
    for line in lines.iter() {
        if line.contains('x') && line.contains(':') {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 2 {
                let dims: Vec<&str> = parts[0].trim().split('x').collect();
                if dims.len() == 2 {
                    let width = dims[0].parse().unwrap();
                    let height = dims[1].parse().unwrap();
                    let required: Vec<usize> = parts[1]
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    regions.push(Region {
                        width,
                        height,
                        required,
                    });
                }
            }
        }
    }

    (shapes, regions)
}

struct Grid {
    width: usize,
    height: usize,
    occupied: Vec<u64>,
    total_cells: usize,
    filled_cells: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let total_cells = width * height;
        let num_words = total_cells.div_ceil(64);
        Grid {
            width,
            height,
            occupied: vec![0; num_words],
            total_cells,
            filled_cells: 0,
        }
    }

    #[inline]
    fn is_occupied(&self, x: usize, y: usize) -> bool {
        let idx = y * self.width + x;
        let word = idx / 64;
        let bit = idx % 64;
        (self.occupied[word] >> bit) & 1 == 1
    }

    #[inline]
    fn set_cell(&mut self, x: usize, y: usize, occupied: bool) {
        let idx = y * self.width + x;
        let word = idx / 64;
        let bit = idx % 64;
        if occupied {
            self.occupied[word] |= 1u64 << bit;
        } else {
            self.occupied[word] &= !(1u64 << bit);
        }
    }

    #[inline]
    fn can_place(&self, shape: &Shape, x: i32, y: i32) -> bool {
        shape.iter().all(|&(dx, dy)| {
            let nx = x + dx;
            let ny = y + dy;
            nx >= 0
                && ny >= 0
                && nx < self.width as i32
                && ny < self.height as i32
                && !self.is_occupied(nx as usize, ny as usize)
        })
    }

    #[inline]
    fn place(&mut self, shape: &Shape, x: i32, y: i32) {
        for &(dx, dy) in shape {
            self.set_cell((x + dx) as usize, (y + dy) as usize, true);
        }
        self.filled_cells += shape.len();
    }

    #[inline]
    fn remove(&mut self, shape: &Shape, x: i32, y: i32) {
        for &(dx, dy) in shape {
            self.set_cell((x + dx) as usize, (y + dy) as usize, false);
        }
        self.filled_cells -= shape.len();
    }
}

/// Backtracking solver to fit all presents into the grid
fn can_fit_presents(
    grid: &mut Grid,
    variants_list: &[Vec<Shape>],
    pieces_to_place: &[(usize, usize)],
    piece_idx: usize,
    pieces_remaining: usize,
    total_cells_needed: usize,
) -> bool {
    // Base case: all pieces placed successfully
    if piece_idx >= pieces_to_place.len() {
        return true;
    }

    // Early termination: impossible to fit remaining pieces
    if grid.total_cells - grid.filled_cells < total_cells_needed {
        return false;
    }

    let (shape_idx, _) = pieces_to_place[piece_idx];
    let shape_variants = &variants_list[shape_idx];
    let cells_per_piece = shape_variants[0].len();

    // Try placing one copy of the current shape at each position
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            for variant in shape_variants {
                if grid.can_place(variant, x, y) {
                    grid.place(variant, x, y);

                    let next_cells_needed = total_cells_needed - cells_per_piece;
                    let success = if pieces_remaining == 1 {
                        // Move to next shape type
                        can_fit_presents(
                            grid,
                            variants_list,
                            pieces_to_place,
                            piece_idx + 1,
                            if piece_idx + 1 < pieces_to_place.len() {
                                pieces_to_place[piece_idx + 1].1
                            } else {
                                0
                            },
                            next_cells_needed,
                        )
                    } else {
                        // Place another copy of the same shape
                        can_fit_presents(
                            grid,
                            variants_list,
                            pieces_to_place,
                            piece_idx,
                            pieces_remaining - 1,
                            next_cells_needed,
                        )
                    };

                    grid.remove(variant, x, y);

                    if success {
                        return true;
                    }
                }
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let (shapes, regions) = parse_input(input);

    // Precompute all shape variants (rotations/flips) once
    let variants_list: Vec<Vec<Shape>> = shapes.iter().map(|s| s.rotations_and_flips()).collect();

    let mut valid_regions = 0;

    for region in regions {
        // Build list of pieces to place: (shape_idx, count)
        let mut pieces_to_place: Vec<(usize, usize)> = region
            .required
            .iter()
            .enumerate()
            .filter(|&(_, &cnt)| cnt > 0)
            .map(|(idx, &cnt)| (idx, cnt))
            .collect();

        // Heuristic: place most numerous pieces first (better pruning)
        pieces_to_place.sort_unstable_by_key(|&(_, cnt)| std::cmp::Reverse(cnt));

        // Calculate total cells needed for early termination
        let total_cells_needed: usize = pieces_to_place
            .iter()
            .map(|&(shape_idx, count)| variants_list[shape_idx][0].len() * count)
            .sum();

        // Quick check: can't fit if more cells needed than available
        if total_cells_needed > region.width * region.height {
            continue;
        }

        let mut grid = Grid::new(region.width, region.height);
        let initial_count = if !pieces_to_place.is_empty() {
            pieces_to_place[0].1
        } else {
            0
        };

        if can_fit_presents(
            &mut grid,
            &variants_list,
            &pieces_to_place,
            0,
            initial_count,
            total_cells_needed,
        ) {
            valid_regions += 1;
        }
    }

    Some(valid_regions)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
