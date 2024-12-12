use std::collections::HashMap;

advent_of_code::solution!(11);

fn solve(cache: &mut HashMap<(u64, usize), u64>, value: u64, steps: usize) -> u64 {
    if let Some(val) = cache.get(&(value, steps)) {
        return *val;
    } else if steps == 0 {
        cache.insert((value, steps), 1);
        return 1;
    }

    if value == 0 {
        solve(cache, 1, steps - 1)
    } else if value.to_string().len() % 2 == 0 {
        let val_str = value.to_string();
        let (left, right) = val_str.split_at(value.to_string().len() / 2);
        let new_val = solve(cache, left.parse::<u64>().unwrap(), steps - 1)
            + solve(cache, right.parse::<u64>().unwrap(), steps - 1);
        cache.insert((value, steps), new_val);
        new_val
    } else {
        solve(cache, value * 2024, steps - 1)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let initial: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    let mut result = 0;

    for num in initial {
        result += solve(&mut cache, num, 25);
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let initial: Vec<u64> = input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut cache: HashMap<(u64, usize), u64> = HashMap::new();

    let mut result = 0;

    for num in initial {
        result += solve(&mut cache, num, 75);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
