advent_of_code::solution!(5);

use std::ops::RangeInclusive;

#[inline]
fn parse_range(s: &str) -> Option<RangeInclusive<u64>> {
    let (start, end) = s.split_once('-')?;
    Some(start.parse().ok()?..=end.parse().ok()?)
}

fn parse_ranges(section: &str) -> Vec<RangeInclusive<u64>> {
    section.lines().filter_map(parse_range).collect()
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_unstable_by_key(|r| *r.start());

    let mut merged = Vec::with_capacity(ranges.len());
    let mut current = ranges[0].clone();

    for range in ranges.into_iter().skip(1) {
        if range.start() <= &(current.end() + 1) {
            current = *current.start()..=(*current.end()).max(*range.end());
        } else {
            merged.push(current);
            current = range;
        }
    }
    merged.push(current);

    merged
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sections = input.split("\n\n");
    let ranges = parse_ranges(sections.next()?);
    let ids: Vec<u64> = sections
        .next()?
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect();

    let merged = merge_ranges(ranges);

    let count = ids
        .iter()
        .filter(|id| merged.iter().any(|range| range.contains(id)))
        .count();

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_ranges(input.split("\n\n").next()?);
    let merged = merge_ranges(ranges);

    Some(merged.iter().map(|r| r.end() - r.start() + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
