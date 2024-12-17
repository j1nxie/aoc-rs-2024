use std::collections::{BinaryHeap, HashMap};

advent_of_code::solution!(16);

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: (i32, i32),
    direction: (i32, i32),
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

fn find_shortest_path(grid: &[Vec<char>]) -> Option<i32> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] == 'S' {
                start = (i, j);
            } else if grid[i as usize][j as usize] == 'E' {
                end = (i, j);
            }
        }
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    let initial_direction = (0, 1);
    heap.push(State {
        cost: 0,
        position: start,
        direction: initial_direction,
    });

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == end {
            return Some(cost);
        }

        let key = (position, direction);
        if let Some(&best) = visited.get(&key) {
            if cost >= best {
                continue;
            }
        }
        visited.insert(key, cost);

        for &new_dir in &directions {
            let new_pos = (position.0 + new_dir.0, position.1 + new_dir.1);

            if new_pos.0 < 0
                || new_pos.0 >= rows
                || new_pos.1 < 0
                || new_pos.1 >= cols
                || grid[new_pos.0 as usize][new_pos.1 as usize] == '#'
            {
                continue;
            }

            let mut new_cost = cost + 1;
            if new_dir != direction {
                new_cost += 1000;
            }

            heap.push(State {
                cost: new_cost,
                position: new_pos,
                direction: new_dir,
            });
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<_>>();

    find_shortest_path(&grid)
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
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
