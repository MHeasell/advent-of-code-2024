use std::{
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

fn solve(input: &Input) -> usize {
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
    // - It seems some paths are more costly than others due to the knock-on
    //   effect on higher-level encodings. It's not obvious to me what the rule
    //   is for this yet.
    input
        .lines
        .iter()
        .map(|l| (l, encode(l)))
        .map(|(code, sequence)| get_complexity(code, &sequence))
        .sum()
}

fn encode(code: &str) -> String {
    code.chars()
        .map(position_of_num_key)
        .scan(position_of_num_key('A'), |pos, next_pos| {
            let result = encode_move_and_press(*pos, next_pos);
            *pos = next_pos;
            Some(result)
        })
        .collect()
}

fn encode_move_and_press(start: Position, end: Position) -> String {
    encodings_for_num_press(start, end)
        .into_iter()
        .flat_map(|seq| encodings_for_dir_seq(&seq))
        .flat_map(|seq| encodings_for_dir_seq(&seq))
        .min_by_key(|s| s.len())
        .unwrap()
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

fn encodings_for_dir_seq(seq: &str) -> Vec<String> {
    encodings_for_dir_seq_inner(position_of_dir_key('A'), seq)
}

fn encodings_for_dir_seq_inner(start_pos: Position, seq: &str) -> Vec<String> {
    let Some(c) = seq.chars().next() else {
        return vec![String::new()];
    };
    let next_pos = position_of_dir_key(c);
    let encoded_seqs = encodings_for_dir_press(start_pos, next_pos);
    encoded_seqs
        .into_iter()
        .flat_map(|s| {
            encodings_for_dir_seq_inner(next_pos, &seq[1..])
                .into_iter()
                .map(move |next| s.clone() + &next)
        })
        .collect()
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

fn get_complexity(code: &str, sequence: &str) -> usize {
    sequence.len() * get_numeric_part(code)
}

fn get_numeric_part(code: &str) -> usize {
    code[0..code.len() - 1].parse().unwrap()
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day21/input").unwrap();
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
029A
980A
179A
456A
379A
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

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

    #[test]
    fn test_encodings_for_dir_seq_inner() {
        assert_eq!(
            encodings_for_dir_seq_inner(position_of_dir_key('A'), "A"),
            vec!["A"]
        );
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("A"), "A");
        assert_eq!(encode("0"), "<vA<AA>>^AvAA<^A>A");
    }
}
