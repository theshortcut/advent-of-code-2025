advent_of_code::solution!(4);

/// The 8 directions for checking neighbors (row_delta, col_delta)
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

/// Count how many paper roll neighbors a position has
#[inline]
fn count_neighbors(grid: &[Vec<u8>], row: usize, col: usize) -> usize {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    DIRECTIONS
        .iter()
        .filter(|(dr, dc)| {
            let nr = row as i32 + dr;
            let nc = col as i32 + dc;
            nr >= 0
                && nr < height
                && nc >= 0
                && nc < width
                && grid[nr as usize][nc as usize] == b'@'
        })
        .count()
}

/// Find all accessible paper rolls (those with fewer than 4 neighbors)
fn find_accessible(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(col, &cell)| {
                if cell == b'@' && count_neighbors(grid, row, col) < 4 {
                    Some((row, col))
                } else {
                    None
                }
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    Some(find_accessible(&grid).len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let mut total = 0;

    loop {
        let accessible = find_accessible(&grid);
        if accessible.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &accessible {
            grid[*row][*col] = b'.';
        }

        total += accessible.len() as u64;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
