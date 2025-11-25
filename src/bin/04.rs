
use std::fmt;
use crypto::digest::Digest;
use crypto::md5::Md5;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let mut cur: u64 = 0;
    let pattern = "00000";
    let mut hash = Md5::new();
    loop
    {
        cur += 1;
        let test = fmt::format(format_args!("{}{}", input, cur));
        hash.input_str(&test);
        let res = hash.result_str();
        if res.starts_with(pattern)
        {
            break;
        }
        hash.reset();
    }
    Some(cur)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cur: u64 = 0;
    let pattern = "000000";
    let mut hash = Md5::new();
    loop
    {
        cur += 1;
        let test = fmt::format(format_args!("{}{}", input, cur));
        hash.input_str(&test);
        let res = hash.result_str();

        if res.starts_with(pattern)
        {
            break;
        }
        hash.reset();
    }
    Some(cur)
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
