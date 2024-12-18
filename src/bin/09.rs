advent_of_code::solution!(9);

fn is_finished(state: &[i32]) -> bool {
    let mut seen = false;
    for c in state {
        if *c == -1 {
            seen = true;
        } else if *c != -1 && seen {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk: Vec<i32> = vec![];
    let mut result: u64 = 0;
    let mut file_id = 0;

    for (idx, c) in input.trim().chars().enumerate() {
        let length = c.to_digit(10).unwrap();

        let blocks: Vec<i32> = match idx % 2 == 0 {
            true => {
                file_id += 1;
                vec![file_id - 1; length as usize]
            }
            false => vec![-1; length as usize],
        };

        for block in blocks {
            disk.push(block);
        }
    }

    while !is_finished(&disk) {
        let left = disk.iter().position(|&i| i == -1);
        let right = disk.iter().rposition(|&i| i != -1);

        match (left, right) {
            (Some(l), Some(r)) if l < r => {
                disk.swap(l, r);
            }
            _ => break,
        }
    }

    for (idx, &num) in disk.iter().enumerate() {
        if num != -1 {
            result += idx as u64 * num as u64;
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
