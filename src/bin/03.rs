use std::sync::LazyLock;

use regex::Regex;

advent_of_code::solution!(3);

static REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let captures = REGEX.captures_iter(input);

    for capture in captures {
        let first = &capture[1].parse::<u32>().unwrap();
        let second = &capture[2].parse::<u32>().unwrap();

        sum += first * second;
    }

    Some(sum)
}

#[derive(Debug)]
enum State {
    Enabled,
    Disabled,
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut state = State::Enabled;
    let mut result = 0;

    let mut chars = input.chars().peekable();

    'outer: while let Some(c) = chars.next() {
        match c {
            'd' => {
                if chars.next() == Some('o') {
                    match chars.next() {
                        Some('(') => {
                            if chars.next() == Some(')') {
                                state = State::Enabled;
                            }
                        }
                        Some('n') => {
                            if chars.next() == Some('\'')
                                && chars.next() == Some('t')
                                && chars.next() == Some('(')
                                && chars.next() == Some(')')
                            {
                                state = State::Disabled;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            'm' => {
                if chars.next() == Some('u')
                    && chars.next() == Some('l')
                    && chars.next() == Some('(')
                {
                    let mut first = String::new();
                    let mut second = String::new();

                    for next_char in chars.by_ref() {
                        if next_char == ',' {
                            break;
                        }

                        if next_char.is_ascii_digit() {
                            first.push(next_char);
                        } else {
                            continue 'outer;
                        }
                    }

                    for next_char in chars.by_ref() {
                        if next_char == ')' {
                            break;
                        }

                        if next_char.is_ascii_digit() {
                            second.push(next_char);
                        } else {
                            continue 'outer;
                        }
                    }

                    if let (Ok(first), Ok(second)) = (first.parse::<u32>(), second.parse::<u32>()) {
                        match state {
                            State::Enabled => result += first * second,
                            State::Disabled => (),
                        }
                    }
                }
            }
            _ => continue,
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
