use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(18);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(obstacles: &[(i32, i32)]) -> Option<i32> {
    let start = (0, 0);
    let end = if cfg!(test) { (6, 6) } else { (70, 70) };

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }

        let key = position;
        if let Some(&best) = visited.get(&key) {
            if cost >= best {
                continue;
            }
        }
        visited.insert(key, cost);

        for &new_dir in &directions {
            let new_pos = (position.0 + new_dir.0, position.1 + new_dir.1);

            if new_pos.0 < 0
                || new_pos.0 > end.0
                || new_pos.1 < 0
                || new_pos.1 > end.1
                || obstacles.contains(&(new_pos.0, new_pos.1))
            {
                continue;
            }

            let new_cost = cost + 1;

            heap.push(State {
                cost: new_cost,
                position: new_pos,
            });
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i32> {
    let obstacles: Vec<(i32, i32)> = input
        .lines()
        .take(if cfg!(test) { 12 } else { 1024 })
        .map(|x| {
            let coords: Vec<i32> = x.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

            (coords[0], coords[1])
        })
        .collect();

    find_shortest_path(&obstacles)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut samples = if cfg!(test) { 12 } else { 1024 };

    loop {
        let obstacles: Vec<(i32, i32)> = input
            .lines()
            .take(samples)
            .map(|x| {
                let coords: Vec<i32> = x.split(',').map(|x| x.parse::<i32>().unwrap()).collect();

                (coords[0], coords[1])
            })
            .collect();

        let result = find_shortest_path(&obstacles);

        if result.is_none() {
            break;
        }

        samples += 1;
    }

    Some(samples as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }
}
