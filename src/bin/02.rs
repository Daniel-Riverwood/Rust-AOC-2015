advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines() {
        let values: Vec<u64> = line.split("x").map(|s| s.parse::<u64>().unwrap()).collect();
        let l = values.first().unwrap_or(&0);
        let w = values.get(1).unwrap_or(&0);
        let h = values.get(2).unwrap_or(&0);
        let v: Vec<u64> = [l * w, w * h, h * l].to_vec();
        let min = v.iter().min().unwrap_or(&0);
        let sum = 2 * l * w + 2 * w * h + 2 * h * l;

        total += min + sum;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total: u64 = 0;
    for line in input.lines() {
        let values: Vec<u64> = line.split("x").map(|s| s.parse::<u64>().unwrap()).collect();
        let l = values.first().unwrap_or(&0);
        let w = values.get(1).unwrap_or(&0);
        let h = values.get(2).unwrap_or(&0);
        let base = l * w * h;
        let sides: Vec<u64> = [l + l + w + w, l + l + h + h, w + w + h + h].to_vec();
        let min = sides.iter().min().unwrap_or(&0);

        total += min + base;
    }

    Some(total)
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
