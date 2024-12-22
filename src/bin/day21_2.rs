use std::{
    collections::HashMap,
    fs::{self},
    iter,
    time::Instant,
};

use aoc24::position::{pos, Position};

#[derive(Debug)]
struct Input {
    lines: Vec<String>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.to_string()).collect::<Vec<_>>();
    Input { lines }
}

fn solve(input: &Input, depth: usize) -> usize {
    // Observations:
    //
    // - When we type a digit, each directional keypad always starts on 'A'.
    //   Only the digit keypad persists state between digit inputs.
    //
    // - Changing input is costly so it's always better to repeat inputs
    //   as much as possible. Therefore there are only max. two ways to
    //   move from one keypad input to another. One of these ways MAY be
    //   illegal due to passing over the illegal square.
    //
    //   Improved observation for part 2:
    //
    // - Keys further from the 'A' key are more costly to press,
    //   because we must travel to the key first.
    //   Repeating a key erases the travel cost for the presses after the first.
    //   Therefore, given the choice of two encodings, we should prefer the encoding
    //   that repeats the most expensive keys.

    // To solve this we will find the encoding with the overall least costly key
    // transitions. We can memoize the cost of a transition from one key to another
    // at a given "encoding depth" to speed things up.
    let mut context = Context::new();

    input
        .lines
        .iter()
        .map(|l| (l, context.get_encoding_len(l, depth)))
        .map(|(code, sequence_len)| get_complexity(code, sequence_len))
        .sum()
}

struct Context {
    cache: HashMap<(char, char, usize), usize>,
}

impl Context {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn get_encoding_len(&mut self, code: &str, depth: usize) -> usize {
        code.chars()
            .map(|c| position_of_num_key(c))
            .scan(position_of_num_key('A'), |pos, next_pos| {
                let result = self.get_move_and_press_encoding_len(*pos, next_pos, depth);
                *pos = next_pos;
                Some(result)
            })
            .sum()
    }

    fn get_move_and_press_encoding_len(
        &mut self,
        start: Position,
        end: Position,
        depth: usize,
    ) -> usize {
        encodings_for_num_press(start, end)
            .into_iter()
            .map(|seq| self.compute_encoding_cost(&seq, depth))
            .min()
            .unwrap()
    }

    fn compute_encoding_cost(&mut self, seq: &str, depth: usize) -> usize {
        seq.chars()
            .scan('A', |prev, next| {
                let cost = self.get_transition_cost(*prev, next, depth);
                *prev = next;
                Some(cost)
            })
            .sum()
    }

    fn get_transition_cost(&mut self, prev: char, next: char, depth: usize) -> usize {
        if depth == 0 {
            return 1;
        }
        if prev == next {
            return 1;
        }

        if let Some(cost) = self.cache.get(&(prev, next, depth)) {
            return *cost;
        }

        let prev_pos = position_of_dir_key(prev);
        let next_pos = position_of_dir_key(next);

        let possible_encodings = encodings_for_dir_press(prev_pos, next_pos);

        let cost = possible_encodings
            .iter()
            .map(|enc| self.compute_encoding_cost(enc, depth - 1))
            .min()
            .unwrap();

        self.cache.insert((prev, next, depth), cost);

        cost
    }
}

fn encodings_for_num_press(start: Position, end: Position) -> Vec<String> {
    let down_distance = end.y - start.y;
    let vertical_seq = if down_distance > 0 {
        iter::repeat_n('v', down_distance as usize)
    } else {
        iter::repeat_n('^', (-down_distance) as usize)
    };

    let right_distance = end.x - start.x;
    let horizontal_seq = if right_distance > 0 {
        iter::repeat_n('>', right_distance as usize)
    } else {
        iter::repeat_n('<', (-right_distance) as usize)
    };

    let path_hv: String = horizontal_seq
        .clone()
        .chain(vertical_seq.clone())
        .chain(iter::once('A'))
        .collect();
    let path_vh: String = vertical_seq
        .chain(horizontal_seq)
        .chain(iter::once('A'))
        .collect();

    if start.x == end.x || start.y == end.y {
        // one axis is the same so both paths are the same,
        // just return one of them.
        return vec![path_hv];
    }

    if start.x == 0 && end.y == 3 {
        // must go horizontal first to avoid illegal square
        return vec![path_hv];
    }
    if start.y == 3 && end.x == 0 {
        // must go vertical first to avoid illegal square
        return vec![path_vh];
    }

    vec![path_hv, path_vh]
}

fn position_of_num_key(k: char) -> Position {
    match k {
        '7' => pos(0, 0),
        '8' => pos(1, 0),
        '9' => pos(2, 0),
        '4' => pos(0, 1),
        '5' => pos(1, 1),
        '6' => pos(2, 1),
        '1' => pos(0, 2),
        '2' => pos(1, 2),
        '3' => pos(2, 2),
        '0' => pos(1, 3),
        'A' => pos(2, 3),
        _ => panic!("illegal num key {}", k),
    }
}

fn encodings_for_dir_press(start: Position, end: Position) -> Vec<String> {
    let down_distance = end.y - start.y;
    let vertical_seq = if down_distance > 0 {
        iter::repeat_n('v', down_distance as usize)
    } else {
        iter::repeat_n('^', (-down_distance) as usize)
    };

    let right_distance = end.x - start.x;
    let horizontal_seq = if right_distance > 0 {
        iter::repeat_n('>', right_distance as usize)
    } else {
        iter::repeat_n('<', (-right_distance) as usize)
    };

    let path_hv: String = horizontal_seq
        .clone()
        .chain(vertical_seq.clone())
        .chain(iter::once('A'))
        .collect();
    let path_vh: String = vertical_seq
        .chain(horizontal_seq)
        .chain(iter::once('A'))
        .collect();

    if start.x == end.x || start.y == end.y {
        // one axis is the same so both paths are the same,
        // just return one of them.
        return vec![path_hv];
    }

    if start.x == 0 && end.y == 0 {
        // must go horizontal first to avoid illegal square
        return vec![path_hv];
    }
    if start.y == 0 && end.x == 0 {
        // must go vertical first to avoid illegal square
        return vec![path_vh];
    }

    vec![path_hv, path_vh]
}

fn position_of_dir_key(c: char) -> Position {
    match c {
        '^' => pos(1, 0),
        'A' => pos(2, 0),
        '<' => pos(0, 1),
        'v' => pos(1, 1),
        '>' => pos(2, 1),
        _ => panic!("illegal dir key {}", c),
    }
}

fn get_complexity(code: &str, sequence_len: usize) -> usize {
    sequence_len * get_numeric_part(code)
}

fn get_numeric_part(code: &str) -> usize {
    code[0..code.len() - 1].parse().unwrap()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day21/input").unwrap();
    let input = parse_input(&input_str);

    let answer = solve(&input, 25);

    println!("Answer: {}", answer);
    println!("Elapsed time: {}us", start_time.elapsed().as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let input_str = "\
029A
980A
179A
456A
379A
";
        let input = parse_input(&input_str);
        let answer = solve(&input, 2);

        assert_eq!(answer, 126384);
    }

    #[test]
    fn test_encodings_for_num_press() {
        assert_eq!(
            encodings_for_num_press(position_of_num_key('A'), position_of_num_key('0')),
            vec!["<A"]
        );
        assert_eq!(
            encodings_for_num_press(position_of_num_key('A'), position_of_num_key('1')),
            vec!["^<<A"]
        );
        assert_eq!(
            encodings_for_num_press(position_of_num_key('1'), position_of_num_key('A')),
            vec![">>vA"]
        );
    }

    #[test]
    fn test_encodings_for_dir_press() {
        assert_eq!(
            encodings_for_dir_press(position_of_dir_key('A'), position_of_dir_key('<')),
            vec!["v<<A"]
        );
    }
}
