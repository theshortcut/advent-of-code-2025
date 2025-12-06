advent_of_code::solution!(6);

struct Problem {
    start_col: usize,
    end_col: usize,
    operator: char,
}

struct Worksheet {
    lines: Vec<Vec<u8>>,
    operator_line: Vec<u8>,
    max_len: usize,
}

impl Worksheet {
    fn parse(input: &str) -> Option<Self> {
        let lines: Vec<&str> = input.lines().collect();
        if lines.len() < 2 {
            return None;
        }

        let max_len = lines.iter().map(|l| l.len()).max()?;
        let operator_line = Self::pad_bytes(lines.last()?.as_bytes(), max_len);

        let padded_lines = lines[..lines.len() - 1]
            .iter()
            .map(|line| Self::pad_bytes(line.as_bytes(), max_len))
            .collect();

        Some(Worksheet {
            lines: padded_lines,
            operator_line,
            max_len,
        })
    }

    fn pad_bytes(bytes: &[u8], len: usize) -> Vec<u8> {
        let mut padded = bytes.to_vec();
        padded.resize(len, b' ');
        padded
    }

    fn has_content_at(&self, col: usize) -> bool {
        self.lines.iter().any(|line| line[col] != b' ')
            || (self.operator_line[col] == b'*' || self.operator_line[col] == b'+')
    }

    fn find_operator(&self, start: usize, end: usize) -> Option<char> {
        self.operator_line[start..end]
            .iter()
            .find(|&&ch| ch == b'*' || ch == b'+')
            .map(|&ch| ch as char)
    }

    fn find_problems(&self) -> Vec<Problem> {
        let mut problems = Vec::new();
        let mut col = 0;

        while col < self.max_len {
            if self.has_content_at(col) {
                let start_col = col;

                while col < self.max_len && self.has_content_at(col) {
                    col += 1;
                }

                if let Some(operator) = self.find_operator(start_col, col) {
                    problems.push(Problem {
                        start_col,
                        end_col: col,
                        operator,
                    });
                }
            } else {
                col += 1;
            }
        }

        problems
    }

    fn evaluate_horizontal(&self, problem: &Problem) -> u64 {
        let numbers: Vec<u64> = self
            .lines
            .iter()
            .filter_map(|line| {
                let slice = &line[problem.start_col..problem.end_col];
                std::str::from_utf8(slice).ok()?.trim().parse::<u64>().ok()
            })
            .collect();

        apply_operator(&numbers, problem.operator)
    }

    fn evaluate_vertical(&self, problem: &Problem) -> u64 {
        let numbers: Vec<u64> = (problem.start_col..problem.end_col)
            .rev()
            .filter_map(|col_idx| {
                // Collect all digits in this column from top to bottom
                let digits: Vec<u8> = self
                    .lines
                    .iter()
                    .filter_map(|line| {
                        let ch = line[col_idx];
                        if ch.is_ascii_digit() { Some(ch) } else { None }
                    })
                    .collect();

                if digits.is_empty() {
                    return None;
                }
                std::str::from_utf8(&digits).ok()?.parse::<u64>().ok()
            })
            .collect();

        apply_operator(&numbers, problem.operator)
    }
}

fn apply_operator(numbers: &[u64], operator: char) -> u64 {
    if numbers.is_empty() {
        return 0;
    }

    match operator {
        '*' => numbers.iter().product(),
        '+' => numbers.iter().sum(),
        _ => 0,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let worksheet = Worksheet::parse(input)?;
    let problems = worksheet.find_problems();

    Some(
        problems
            .iter()
            .map(|problem| worksheet.evaluate_horizontal(problem))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let worksheet = Worksheet::parse(input)?;
    let problems = worksheet.find_problems();

    Some(
        problems
            .iter()
            .map(|problem| worksheet.evaluate_vertical(problem))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
