use std::{
    fs::{self},
    time::Instant,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    vm: (i64, i64, i64),
    program: Vec<u8>,
}

lazy_static! {
    static ref REGISTER_A_REGEX: Regex = Regex::new(r"^Register A: (\d+)$").unwrap();
    static ref REGISTER_B_REGEX: Regex = Regex::new(r"^Register B: (\d+)$").unwrap();
    static ref REGISTER_C_REGEX: Regex = Regex::new(r"^Register C: (\d+)$").unwrap();
    static ref PROGRAM_REGEX: Regex = Regex::new(r"^Program: (\d(?:,\d)+)$").unwrap();
}

fn parse_input(s: &str) -> Input {
    let mut lines = s.lines();
    let reg_a = lines
        .next()
        .map(|l| REGISTER_A_REGEX.captures(l).unwrap()[1].parse().unwrap())
        .unwrap();
    let reg_b = lines
        .next()
        .map(|l| REGISTER_B_REGEX.captures(l).unwrap()[1].parse().unwrap())
        .unwrap();
    let reg_c = lines
        .next()
        .map(|l| REGISTER_C_REGEX.captures(l).unwrap()[1].parse().unwrap())
        .unwrap();
    assert_eq!(lines.next(), Some(""));
    let program = lines
        .next()
        .map(|l| {
            PROGRAM_REGEX.captures(l).unwrap()[1]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .unwrap();
    Input {
        vm: (reg_a, reg_b, reg_c),
        program,
    }
}

struct VmState {
    a: i64,
    b: i64,
    c: i64,
    pc: usize,
}

impl VmState {
    fn get_combo(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("illegal"),
        }
    }
}

fn solve(input: &Input) -> String {
    let mut vm = VmState {
        a: input.vm.0,
        b: input.vm.1,
        c: input.vm.2,
        pc: 0,
    };
    let mut output = Vec::<u8>::new();
    let code = &input.program;

    while vm.pc < code.len() - 1 {
        // decode and execute instruction
        let op = code[vm.pc];
        let operand = code[vm.pc + 1];
        vm.pc += 2;

        match op {
            // adv
            0 => {
                vm.a /= 2i64.pow(vm.get_combo(operand).try_into().unwrap());
            }
            // bxl
            1 => {
                vm.b ^= i64::from(operand);
            }
            // bst
            2 => {
                vm.b = vm.get_combo(operand) % 8;
            }
            // jnz
            3 => {
                if vm.a != 0 {
                    vm.pc = operand.into();
                }
            }
            // bxc
            4 => {
                vm.b ^= vm.c;
            }
            // out
            5 => {
                let val = vm.get_combo(operand) % 8;
                output.push(val as u8);
            }
            // bdv
            6 => {
                vm.b = vm.a / 2i64.pow(vm.get_combo(operand).try_into().unwrap());
            }
            // cdv
            7 => {
                vm.c = vm.a / 2i64.pow(vm.get_combo(operand).try_into().unwrap());
            }
            _ => panic!("illegal opcode {}", op),
        }
    }

    output
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day17/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, "4,6,3,5,6,3,5,2,1,0");
    }
}
