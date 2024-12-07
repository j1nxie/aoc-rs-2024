use std::collections::HashSet;

advent_of_code::solution!(6);

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

struct StateMachine {
    position: (i32, i32),
    current_direction: Direction,
    obstacles: Vec<(i32, i32)>,
    stepped: HashSet<(i32, i32)>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            position: (0, 0),
            current_direction: Direction::Up,
            obstacles: vec![],
            stepped: HashSet::new(),
        }
    }

    fn step(&mut self) {
        match self.current_direction {
            Direction::Up => self.position = (self.position.0, self.position.1 - 1),
            Direction::Down => self.position = (self.position.0, self.position.1 + 1),
            Direction::Left => self.position = (self.position.0 - 1, self.position.1),
            Direction::Right => self.position = (self.position.0 + 1, self.position.1),
        }

        for obstacle in self.obstacles.clone() {
            if self.position == obstacle {
                self.back();
                self.current_direction = self.current_direction.next();
            }
        }

        self.stepped.insert(self.position);
    }

    fn back(&mut self) {
        match self.current_direction {
            Direction::Up => self.position = (self.position.0, self.position.1 + 1),
            Direction::Down => self.position = (self.position.0, self.position.1 - 1),
            Direction::Left => self.position = (self.position.0 + 1, self.position.1),
            Direction::Right => self.position = (self.position.0 - 1, self.position.1),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut state = StateMachine::new();

    for (idy, line) in input.lines().enumerate() {
        for (idx, ch) in line.chars().enumerate() {
            if ch == '#' {
                state.obstacles.push((idx as i32, idy as i32));
            }

            if ch == '^' {
                state.position = (idx as i32, idy as i32);
                state.stepped.insert(state.position);
            }
        }
    }

    let width = input.lines().collect::<Vec<_>>().len() as i32;
    let height = input.lines().next().unwrap().len() as i32;

    loop {
        state.step();

        if state.position.0 > width - 1
            || state.position.0 < 0
            || state.position.1 > height - 1
            || state.position.1 < 0
        {
            state.stepped.remove(&state.position);
            break;
        }
    }

    Some(state.stepped.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let width = input.lines().collect::<Vec<_>>().len() as i32;
    let height = input.lines().next().unwrap().len() as i32;

    let mut original_obstacles = Vec::new();
    let mut start_pos = (0, 0);

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                original_obstacles.push((x as i32, y as i32));
            }
            if ch == '^' {
                start_pos = (x as i32, y as i32);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
            if original_obstacles.contains(&pos) || pos == start_pos {
                continue;
            }

            let mut state = StateMachine::new();
            state.position = start_pos;
            state.obstacles = original_obstacles.clone();
            state.obstacles.push(pos);
            state.stepped.insert(start_pos);

            let mut steps = 0;
            let max_steps = (width * height) as usize;
            let mut is_loop = true;

            while steps < max_steps {
                steps += 1;
                state.step();

                if state.position.0 >= width
                    || state.position.0 < 0
                    || state.position.1 >= height
                    || state.position.1 < 0
                {
                    is_loop = false;
                    break;
                }
            }

            if is_loop {
                result += 1;
            }
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
