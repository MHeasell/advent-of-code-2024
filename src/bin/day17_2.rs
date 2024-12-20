use std::{
    fs::{self},
    time::Instant,
};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Input {
    #[allow(dead_code)]
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

fn solve(input: &Input) -> i64 {
    // hand-written disassembly of the VM code:
    //
    // bst 'a    // b = a % 8
    // bxl 5     // b ^= 5
    // cdv 'b    // c = a / (2**b)
    // bxl 6     // b ^= 6
    // bxc       // b ^= c
    // out 'b    // print b
    // adv 3     // a /= 8
    // jnz 0     // if a != 0 then goto start
    //
    // So we are basically consuming the last 3 bits of 'a,
    // doing some bit twiddling on it and outputting it,
    // repeatedly in a loop.
    // We can go backwards then and figure out how to build up 'a
    // to build up the output we want, starting from the last output.
    let code = &input.program;
    find_reg_val(&code, 0).unwrap()
}

fn find_reg_val(outs: &[u8], final_reg_val: i64) -> Option<i64> {
    if outs.is_empty() {
        return Some(final_reg_val);
    }

    let out = outs[outs.len() - 1];

    let mult_reg = final_reg_val * 8;
    (0..8)
        .map(|i| mult_reg + i)
        .filter(|i| *i != 0) // can't be 0 because loop terminates when it's 0
        .filter(|r| compute_out(*r) == out)
        .find_map(|new_r| find_reg_val(&outs[..(outs.len() - 1)], new_r))
}

fn compute_out(a: i64) -> u8 {
    let mut b = a % 8;
    b ^= 5;
    let c = a / 2i64.pow(b.try_into().unwrap());
    b ^= 6;
    b ^= c;
    (b % 8).try_into().unwrap()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day17/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}ms", start_time.elapsed().as_millis());
}
