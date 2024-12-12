use std::collections::HashMap;

advent_of_code::solution!(7);

fn evaluate(nums: &[u64], ops: &[char]) -> u64 {
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            '|' => result = format!("{}{}", result, nums[i + 1]).parse::<u64>().unwrap(),
            _ => unreachable!(),
        }
    }
    result
}

fn check(target: u64, nums: &[u64]) -> bool {
    let gaps = nums.len() - 1;

    for i in 0..(1 << gaps) {
        let mut current_ops = vec![];
        for j in 0..gaps {
            current_ops.push(if (i & (1 << j)) == 0 { '+' } else { '*' });
        }

        if evaluate(nums, &current_ops) == target {
            return true;
        }
    }

    false
}

fn check_part2(target: u64, nums: &[u64]) -> bool {
    let gaps = nums.len() - 1;

    for i in 0..(3_u64.pow(gaps as u32)) {
        let mut current_ops = vec![];
        let mut num = i;

        for _ in 0..gaps {
            let op = match num % 3 {
                0 => '+',
                1 => '*',
                2 => '|',
                _ => unreachable!(),
            };
            current_ops.push(op);
            num /= 3;
        }

        if evaluate(nums, &current_ops) == target {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let target = parts[0].trim().parse::<u64>().unwrap();
            let numbers: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (target, numbers)
        })
        .collect();

    for (target, nums) in data {
        if check(target, &nums) {
            result += target;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let data: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let target = parts[0].trim().parse::<u64>().unwrap();
            let numbers: Vec<u64> = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect();
            (target, numbers)
        })
        .collect();

    for (target, nums) in data {
        if check_part2(target, &nums) {
            result += target;
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
