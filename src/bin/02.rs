advent_of_code::solution!(2);

use std::ops::RangeInclusive;

/// Parse a range string like "11-22" into a RangeInclusive
fn parse_range(s: &str) -> Option<RangeInclusive<u64>> {
    let (start_str, end_str) = s.split_once('-')?;
    let start = start_str.parse().ok()?;
    let end = end_str.parse().ok()?;
    Some(start..=end)
}

/// Check if a number is made of exactly two repetitions of a pattern
#[inline]
fn has_two_repetitions(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Must be even length to split into two equal parts
    if !len.is_multiple_of(2) {
        return false;
    }

    let mid = len / 2;
    let (first, second) = s.split_at(mid);

    !first.starts_with('0') && first == second
}

/// Check if a number is made of any pattern repeated at least twice
#[inline]
fn has_repeating_pattern(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    for pattern_len in 1..=(len / 2) {
        if !len.is_multiple_of(pattern_len) {
            continue;
        }

        let pattern = &s[..pattern_len];

        // Pattern can't have leading zeros
        if pattern.starts_with('0') {
            continue;
        }

        // Check if all chunks equal the pattern (avoids allocating via repeat())
        let pattern_bytes = pattern.as_bytes();
        if s.as_bytes()
            .chunks(pattern_len)
            .all(|chunk| chunk == pattern_bytes)
        {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum: u64 = input
        .trim()
        .split(',')
        .filter_map(parse_range)
        .flatten()
        .filter(|&n| has_two_repetitions(n))
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let sum: u64 = input
        .trim()
        .split(',')
        .filter_map(parse_range)
        .flatten()
        .filter(|&n| has_repeating_pattern(n))
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
