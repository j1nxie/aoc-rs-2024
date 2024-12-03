advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    let mut sum: u32 = 0;

    let _: Vec<_> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            left.push(numbers[0]);
            right.push(numbers[1]);
        })
        .collect();

    left.sort();
    right.sort();

    for (idx, num) in left.iter().enumerate() {
        let diff = if *num > right[idx] {
            num - right[idx]
        } else {
            right[idx] - num
        };

        sum += diff;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left = vec![];
    let mut right = vec![];

    let mut sum: u32 = 0;

    let _: Vec<_> = input
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            left.push(numbers[0]);
            right.push(numbers[1]);
        })
        .collect();

    left.iter().for_each(|num| {
        let occurences = right.iter().filter(|x| *x == num).collect::<Vec<_>>().len();

        sum += num * occurences as u32;
    });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
