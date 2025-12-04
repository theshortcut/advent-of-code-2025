advent_of_code::solution!(3);

/// Find the maximum joltage by selecting exactly `count` batteries using a greedy algorithm.
///
/// Strategy: For each position (left to right), choose the largest available digit
/// from the valid range while ensuring enough batteries remain for subsequent positions.
/// When ties occur, we select the leftmost maximum to preserve flexibility.
#[inline]
fn max_joltage(bank: &str, count: usize) -> u64 {
    // Parse digits directly from bytes for efficiency
    let digits: Vec<u8> = bank
        .bytes()
        .filter(|&b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();

    if digits.len() < count {
        return 0;
    }

    let mut result = 0u64;
    let mut position = 0;

    for i in 0..count {
        // Calculate the search window: must leave enough digits for remaining positions
        let remaining_needed = count - i - 1;
        let window_end = digits.len() - remaining_needed;

        // Find the first occurrence of the maximum digit in this window
        let max_digit = *digits[position..window_end].iter().max().unwrap();
        let offset = digits[position..window_end]
            .iter()
            .position(|&d| d == max_digit)
            .unwrap();

        result = result * 10 + max_digit as u64;
        position += offset + 1;
    }

    result
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| max_joltage(line, 2)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|line| max_joltage(line, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
