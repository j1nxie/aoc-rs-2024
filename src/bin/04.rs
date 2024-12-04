advent_of_code::solution!(4);

fn check_word(start_row: i32, start_col: i32, dir: &(i32, i32), grid: &[Vec<char>]) -> bool {
    let word = "XMAS";
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let (dx, dy) = dir;

    for (i, target_char) in word.chars().enumerate() {
        let row = start_row + (dy * i as i32);
        let col = start_col + (dx * i as i32);

        if row < 0 || row >= height || col < 0 || col >= width {
            return false;
        }

        if grid[row as usize][col as usize] != target_char {
            return false;
        }
    }

    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    let dirs = vec![
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];

    for row in 0..height {
        for col in 0..width {
            for dir in &dirs {
                if check_word(row as i32, col as i32, dir, &map) {
                    result += 1;
                }
            }
        }
    }

    Some(result)
}

fn check_mas(
    start_row: usize,
    start_col: usize,
    pattern: &[(usize, usize, char)],
    grid: &[Vec<char>],
) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    if start_row + 2 >= height || start_col + 2 >= width {
        return false;
    }

    for &(dy, dx, expected) in pattern {
        if grid[start_row + dy][start_col + dx] != expected {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let patterns = vec![
        vec![
            (0, 0, 'M'),
            (0, 2, 'S'),
            (1, 1, 'A'),
            (2, 0, 'M'),
            (2, 2, 'S'),
        ],
        vec![
            (0, 0, 'M'),
            (0, 2, 'M'),
            (1, 1, 'A'),
            (2, 0, 'S'),
            (2, 2, 'S'),
        ],
        vec![
            (0, 0, 'S'),
            (0, 2, 'M'),
            (1, 1, 'A'),
            (2, 0, 'S'),
            (2, 2, 'M'),
        ],
        vec![
            (0, 0, 'S'),
            (0, 2, 'S'),
            (1, 1, 'A'),
            (2, 0, 'M'),
            (2, 2, 'M'),
        ],
    ];

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            for pattern in &patterns {
                if check_mas(row, col, pattern, &map) {
                    result += 1;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
