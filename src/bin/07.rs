advent_of_code::solution!(7);

use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a tachyon manifold with splitters
struct Manifold {
    grid: Vec<Vec<u8>>,
    rows: usize,
    cols: usize,
    start_col: usize,
}

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to parse manifold: empty grid or no start position"
        )
    }
}

impl std::error::Error for ParseError {}

impl TryFrom<&str> for Manifold {
    type Error = ParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        if grid.is_empty() {
            return Err(ParseError);
        }

        let rows = grid.len();
        let cols = grid[0].len();

        // Find the starting position 'S' using iterator methods
        let start_col = grid[0]
            .iter()
            .position(|&ch| ch == b'S')
            .ok_or(ParseError)?;

        Ok(Manifold {
            grid,
            rows,
            cols,
            start_col,
        })
    }
}

impl Manifold {
    /// Check if a position contains a splitter
    #[inline]
    fn is_splitter(&self, row: usize, col: usize) -> bool {
        self.grid[row][col] == b'^'
    }

    /// Find the next splitter in a column starting from a given row
    /// Returns None if the beam exits the grid without hitting a splitter
    fn find_next_splitter(&self, start_row: usize, col: usize) -> Option<usize> {
        (start_row..self.rows).find(|&row| self.is_splitter(row, col))
    }

    /// Count beam splits in a classical manifold (Part 1)
    /// Returns the number of unique splitters encountered
    fn count_classical_splits(&self) -> u64 {
        let mut processed_splitters = HashSet::new();
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        // Start with a beam at the starting position
        queue.push_back((0, self.start_col));
        seen.insert((0, self.start_col));

        while let Some((row, col)) = queue.pop_front() {
            if let Some(splitter_row) = self.find_next_splitter(row, col) {
                // Only process if we haven't seen this splitter before
                if processed_splitters.insert((splitter_row, col)) {
                    // Create two new beams from immediate left and right
                    if col > 0 && seen.insert((splitter_row, col - 1)) {
                        queue.push_back((splitter_row, col - 1));
                    }
                    if col + 1 < self.cols && seen.insert((splitter_row, col + 1)) {
                        queue.push_back((splitter_row, col + 1));
                    }
                }
            }
        }

        processed_splitters.len() as u64
    }

    /// Count timelines in a quantum manifold (Part 2)
    /// Returns the number of distinct paths through the manifold
    fn count_quantum_timelines(&self) -> u64 {
        let mut memo = HashMap::new();
        self.count_timelines_recursive(0, self.start_col, &mut memo)
    }

    /// Recursively count timelines with memoization
    fn count_timelines_recursive(
        &self,
        row: usize,
        col: usize,
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        // Check memoization cache
        if let Some(&cached) = memo.get(&(row, col)) {
            return cached;
        }

        let result = if let Some(splitter_row) = self.find_next_splitter(row, col) {
            // Hit a splitter - quantum split into both paths
            let left_count = if col > 0 {
                self.count_timelines_recursive(splitter_row, col - 1, memo)
            } else {
                0
            };

            let right_count = if col + 1 < self.cols {
                self.count_timelines_recursive(splitter_row, col + 1, memo)
            } else {
                0
            };

            left_count + right_count
        } else {
            // Exited the grid - one complete timeline
            1
        };

        memo.insert((row, col), result);
        result
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let manifold = Manifold::try_from(input).ok()?;
    Some(manifold.count_classical_splits())
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = Manifold::try_from(input).ok()?;
    Some(manifold.count_quantum_timelines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
