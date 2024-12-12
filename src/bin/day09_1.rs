use std::{
    fs::{self},
    iter,
    time::Instant,
};

#[derive(Debug)]
struct Input {
    line: Vec<u8>,
}

fn parse_input(s: &str) -> Input {
    let line = s
        .lines()
        .next()
        .map(|l| l.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .unwrap();
    Input { line }
}

fn solve(input: &Input) -> usize {
    // You can definitely save memory by not expanding out the runs and writing
    // a fancier algorithm. I did noodle with doing that but I found it too
    // fiddly for a first pass so I went back to the dumb brute force algo
    // instead. The full expanded array is only like ~200k elems.
    let mut blocks = expand(&input.line);
    compact(&mut blocks);
    compute_checksum(&blocks)
}

fn expand(line: &[u8]) -> Vec<Option<usize>> {
    line.chunks(2)
        .enumerate()
        .flat_map(|(i, c)| {
            iter::repeat_n(Some(i), c[0].into())
                .chain(iter::repeat_n(None, c.get(1).copied().unwrap_or(0).into()))
        })
        .collect()
}

fn compute_checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, id)| id.map(|id| pos * id))
        .sum()
}

fn compact(blocks: &mut [Option<usize>]) {
    let mut idx = 0;
    let mut end_idx = blocks.len() - 1;

    while idx < end_idx {
        let Some(val) = blocks[end_idx] else {
            end_idx -= 1;
            continue;
        };

        if blocks[idx].is_some() {
            idx += 1;
            continue;
        }

        blocks[idx] = Some(val);
        blocks[end_idx] = None;
        idx += 1;
        end_idx -= 1;
    }
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day09/input").unwrap();
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
2333133121414131402
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 1928);
    }
}
