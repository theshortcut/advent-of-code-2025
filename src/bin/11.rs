use std::collections::HashMap;

advent_of_code::solution!(11);

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

const DAC_BIT: u8 = 1;
const FFT_BIT: u8 = 2;
const BOTH_REQUIRED: u8 = DAC_BIT | FFT_BIT;

fn parse_graph(input: &str) -> Graph<'_> {
    input
        .lines()
        .filter_map(|line| {
            let (node, neighbors) = line.split_once(':')?;
            let neighbors = neighbors.split_whitespace().collect();
            Some((node.trim(), neighbors))
        })
        .collect()
}

fn count_paths<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    target: &str,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if current == target {
        return 1;
    }

    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    let result = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|&neighbor| count_paths(graph, neighbor, target, memo))
                .sum()
        })
        .unwrap_or(0);

    memo.insert(current, result);
    result
}

fn count_paths_with_required<'a>(
    graph: &Graph<'a>,
    current: &'a str,
    target: &str,
    state: u8,
    memo: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    if current == target {
        return if state == BOTH_REQUIRED { 1 } else { 0 };
    }

    let key = (current, state);
    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    let result = graph
        .get(current)
        .map(|neighbors| {
            neighbors
                .iter()
                .map(|&neighbor| {
                    let new_state = state
                        | if neighbor == "dac" { DAC_BIT } else { 0 }
                        | if neighbor == "fft" { FFT_BIT } else { 0 };
                    count_paths_with_required(graph, neighbor, target, new_state, memo)
                })
                .sum()
        })
        .unwrap_or(0);

    memo.insert(key, result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memo = HashMap::new();
    Some(count_paths(&graph, "you", "out", &mut memo))
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph = parse_graph(input);
    let mut memo = HashMap::new();
    Some(count_paths_with_required(
        &graph, "svr", "out", 0, &mut memo,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
