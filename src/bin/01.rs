advent_of_code::solution!(1);

struct Rotation {
    direction: char,
    distance: i32,
}

pub fn part_one(input: &str) -> Option<u32> {
    let rotations = input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction = direction.chars().next().unwrap();
            let distance = distance.parse().unwrap();
            return Rotation {
                direction,
                distance,
            };
        })
        .collect::<Vec<_>>();

    let mut dial: i32 = 50;
    let mut zeroes = 0;
    for rotation in rotations {
        match rotation.direction {
            'L' => dial = (dial + rotation.distance).rem_euclid(100),
            'R' => dial = (dial - rotation.distance).rem_euclid(100),
            _ => unreachable!(),
        }
        if dial == 0 {
            zeroes += 1;
        }
    }
    Some(zeroes)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
