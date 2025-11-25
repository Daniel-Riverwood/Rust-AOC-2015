use std::collections::HashMap;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut curloc = (0, 0);
    let mut map = HashMap::new();
    map.insert(curloc, 1);

    for c in input.chars()
    {
        match c {
            '^' => { curloc.0 += 1; }
            '>' => { curloc.1 += 1; }
            'v' => { curloc.0 -= 1; }
            '<' => { curloc.1 -= 1; }
            _ => {}
        }

        match map.get(&curloc) {
            Some(count) => { map.insert(curloc, count + 1); }
            None => { map.insert(curloc, 1); }
        }
    }

    Some(map.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut curloc = (0, 0);
    let mut botloc = (0, 0);
    let mut map = HashMap::new();
    map.insert(curloc, 1);

    for (i, c) in input.chars().enumerate()
    {
        let mut curapplied = if i % 2 == 0 { curloc } else { botloc };

        match c {
            '^' => { curapplied.0 += 1; }
            '>' => { curapplied.1 += 1; }
            'v' => { curapplied.0 -= 1; }
            '<' => { curapplied.1 -= 1; }
            _ => {}
        }

        match map.get(&curapplied) {
            Some(count) => { map.insert(curapplied, count + 1); }
            None => { map.insert(curapplied, 1); }
        }

        if i % 2 == 0 {
            curloc = curapplied;
        }
        else {
            botloc = curapplied;
        }
    }

    Some(map.len() as u64)
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
