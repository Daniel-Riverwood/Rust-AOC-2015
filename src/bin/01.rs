advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut floor: i64 = 0;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
    }

    Some(floor)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut floor = 0;
    let mut res = 0;
    for (i, c) in input.chars().enumerate() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }

        if floor < 0 {
            res = i + 1;
            break;
        }
    }

    Some(res as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.is_some(), true);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.is_some(), true);
    }
}
