advent_of_code::solution!(2);

fn check(report: Vec<u32>) -> bool {
    let sorted = report.windows(2).all(|w| w[0] <= w[1]) || report.windows(2).all(|w| w[0] >= w[1]);

    let diff = report
        .windows(2)
        .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3);

    sorted && diff
}

fn check_remove(report: Vec<u32>) -> bool {
    for (idx, _) in report.iter().enumerate() {
        let mut new_report = report.to_vec();

        new_report.remove(idx);

        if check(new_report) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let reports: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    for report in reports {
        if check(report) {
            sum += 1;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    let reports: Vec<_> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    for report in reports {
        if check_remove(report) {
            sum += 1;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
