use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(17);

struct StateMachine {
    pc: usize,
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    output: Vec<u64>,
}

impl StateMachine {
    fn new() -> Self {
        Self {
            pc: 0,
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            output: vec![],
        }
    }

    fn run_program(&mut self, program: &[u64]) {
        loop {
            if self.pc >= program.len() {
                break;
            }

            match program[self.pc] {
                0 => {
                    self.reg_a /= 2u64.pow(self.get_combo_operand(program[self.pc + 1]) as u32);
                    self.pc += 2;
                }
                1 => {
                    self.reg_b ^= program[self.pc + 1];
                    self.pc += 2;
                }
                2 => {
                    self.reg_b = self.get_combo_operand(program[self.pc + 1]) % 8;
                    self.pc += 2;
                }
                3 => {
                    if self.reg_a == 0 {
                        self.pc += 2;
                        continue;
                    } else {
                        self.pc = program[self.pc + 1] as usize;
                    }
                }
                4 => {
                    self.reg_b ^= self.reg_c;
                    self.pc += 2;
                }
                5 => {
                    self.output
                        .push(self.get_combo_operand(program[self.pc + 1]) % 8);
                    self.pc += 2;
                }
                6 => {
                    self.reg_b =
                        self.reg_a / 2u64.pow(self.get_combo_operand(program[self.pc + 1]) as u32);
                    self.pc += 2;
                }
                7 => {
                    self.reg_c =
                        self.reg_a / 2u64.pow(self.get_combo_operand(program[self.pc + 1]) as u32);
                    self.pc += 2;
                }
                _ => unreachable!(),
            }
        }
    }

    fn get_combo_operand(&self, val: u64) -> u64 {
        match val {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut state = StateMachine::new();

    for line in input.lines() {
        if line.starts_with("Register A:") {
            state.reg_a = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register B:") {
            state.reg_b = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();
        } else if line.starts_with("Register C:") {
            state.reg_c = line
                .split(':')
                .nth(1)
                .unwrap()
                .trim()
                .parse::<u64>()
                .unwrap();
        }
    }

    let program: Vec<u64> = input
        .lines()
        .find(|line| line.starts_with("Program:"))
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect();

    state.run_program(&program);

    let result = state
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",");

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let program: Vec<u64> = input
        .lines()
        .find(|line| line.starts_with("Program:"))
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|num| num.trim().parse::<u64>().unwrap())
        .collect();

    ((1 << 45)..(1 << 48)).into_par_iter().find_first(|i| {
        let mut state = StateMachine::new();

        state.reg_a = *i;

        state.run_program(&program);

        state.output == program
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("4,6,3,5,6,3,5,2,1,0")));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
