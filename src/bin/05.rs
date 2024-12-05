use std::cmp::Ordering;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut pairs = vec![];
    let mut updates = vec![];
    let mut ctx = false;
    let mut result = 0;

    for line in lines {
        if line.is_empty() {
            ctx = true;
            continue;
        }

        if ctx {
            let numbers = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            updates.push(numbers);
            continue;
        }

        let pair = line
            .split_once('|')
            .map(|pair| {
                (
                    pair.0.parse::<u32>().unwrap(),
                    pair.1.parse::<u32>().unwrap(),
                )
            })
            .unwrap();

        pairs.push(pair);
    }

    for update in updates {
        let mut sorted = update.clone();

        sorted.sort_by(|a, b| {
            let rule = pairs
                .iter()
                .filter(|pair| pair.0 == *a && pair.1 == *b || pair.0 == *b && pair.1 == *a)
                .collect::<Vec<_>>();

            if rule[0].0 == *a {
                return Ordering::Less;
            }

            if rule[0].0 == *b {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        if update
            .iter()
            .enumerate()
            .all(|(idx, page)| *page == sorted[idx])
        {
            result += update[update.len() / 2];
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut pairs = vec![];
    let mut updates = vec![];
    let mut ctx = false;
    let mut result = 0;

    for line in lines {
        if line.is_empty() {
            ctx = true;
            continue;
        }

        if ctx {
            let numbers = line
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            updates.push(numbers);
            continue;
        }

        let pair = line
            .split_once('|')
            .map(|pair| {
                (
                    pair.0.parse::<u32>().unwrap(),
                    pair.1.parse::<u32>().unwrap(),
                )
            })
            .unwrap();

        pairs.push(pair);
    }

    for mut update in updates {
        let mut sorted = update.clone();

        sorted.sort_by(|a, b| {
            let rule = pairs
                .iter()
                .filter(|pair| pair.0 == *a && pair.1 == *b || pair.0 == *b && pair.1 == *a)
                .collect::<Vec<_>>();

            if rule[0].0 == *a {
                return Ordering::Less;
            }

            if rule[0].0 == *b {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        if !update
            .iter()
            .enumerate()
            .all(|(idx, page)| *page == sorted[idx])
        {
            update.sort_by(|a, b| {
                let rule = pairs
                    .iter()
                    .filter(|pair| pair.0 == *a && pair.1 == *b || pair.0 == *b && pair.1 == *a)
                    .collect::<Vec<_>>();

                if rule[0].0 == *a {
                    return Ordering::Less;
                }

                if rule[0].0 == *b {
                    return Ordering::Greater;
                }

                Ordering::Equal
            });

            result += update[update.len() / 2];
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
