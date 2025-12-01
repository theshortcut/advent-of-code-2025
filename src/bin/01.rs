advent_of_code::solution!(1);

use std::str::FromStr;

struct Rotation {
    direction: char,
    distance: i32,
}

impl Rotation {
    fn apply(&self, current: i32) -> i32 {
        match self.direction {
            'L' => (current + self.distance).rem_euclid(100),
            'R' => (current - self.distance).rem_euclid(100),
            _ => unreachable!(),
        }
    }
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.chars().next().ok_or("Empty line")?;
        let distance = s[1..].parse().map_err(|e| format!("Parse error: {}", e))?;
        Ok(Rotation {
            direction,
            distance,
        })
    }
}

fn count_zero_crossings(current: i32, direction: char, distance: i32) -> u64 {
    if distance == 0 || current == 0 {
        return (distance / 100) as u64;
    }

    match direction {
        'L' => {
            // Moving left (toward higher numbers): cross 0 at 100-current, 200-current, ...
            let first_zero = 100 - current;
            if distance >= first_zero {
                ((distance - first_zero) / 100 + 1) as u64
            } else {
                0
            }
        }
        'R' => {
            // Moving right (toward lower numbers): cross 0 at current, 100+current, ...
            if distance >= current {
                ((distance - current) / 100 + 1) as u64
            } else {
                0
            }
        }
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, count) = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<Rotation>().ok())
        .fold((50, 0), |(dial, count), rotation| {
            let new_dial = rotation.apply(dial);
            let new_count = count + (new_dial == 0) as u32;
            (new_dial, new_count)
        });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, count) = input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse::<Rotation>().ok())
        .fold((50, 0), |(dial, count), rotation| {
            let crossings = count_zero_crossings(dial, rotation.direction, rotation.distance);
            let new_dial = rotation.apply(dial);
            (new_dial, count + crossings)
        });

    Some(count)
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
        assert_eq!(result, Some(6));
    }
}
