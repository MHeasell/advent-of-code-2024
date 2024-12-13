use std::{
    fs::{self},
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
    let runs = to_runs(&input.line);
    let mut files = runs
        .iter()
        .scan(0usize, |pos, r| {
            let start = *pos;
            *pos += usize::from(r.len);
            Some((start, r))
        })
        .filter_map(|(p, r)| r.file_id.map(|id| (p, id, r.len)))
        .collect::<Vec<_>>();

    let mut frees = runs
        .iter()
        .scan(0usize, |pos, r| {
            let start = *pos;
            *pos += usize::from(r.len);
            Some((start, r))
        })
        .filter_map(|(p, r)| r.file_id.is_none().then_some((p, r.len)))
        .collect::<Vec<_>>();

    compact(&mut files, &mut frees);
    compute_checksum(&files)
}

fn compute_checksum(files: &[(usize, usize, u8)]) -> usize {
    files
        .iter()
        .map(|(pos, id, len)| compute_file_checksum(*pos, *id, *len))
        .sum()
}

fn compute_file_checksum(pos: usize, file_id: usize, len: u8) -> usize {
    let tri = triangle(usize::from(len) - 1);
    (tri * file_id) + (pos * usize::from(len) * file_id)
}

fn compact(files: &mut [(usize, usize, u8)], frees: &mut [(usize, u8)]) {
    files.iter_mut().rev().for_each(|(file_pos, _, file_len)| {
        let free = frees
            .iter_mut()
            .take_while(|(pos, _)| *pos < *file_pos)
            .find(|(_, len)| *len >= *file_len);
        if let Some((free_pos, free_len)) = free {
            *file_pos = *free_pos;
            *free_pos += usize::from(*file_len);
            *free_len -= *file_len;
        }
    });
}

#[derive(Debug, Clone, Copy)]
struct Run {
    len: u8,
    file_id: Option<usize>,
}

impl Run {
    fn file(len: u8, file_id: usize) -> Self {
        Run {
            len,
            file_id: Some(file_id),
        }
    }
    fn free(len: u8) -> Self {
        Run { len, file_id: None }
    }
}

fn to_runs(line: &[u8]) -> Vec<Run> {
    line.chunks(2)
        .enumerate()
        .flat_map(|(i, c)| {
            [Some(Run::file(c[0], i)), c.get(1).copied().map(Run::free)]
                .into_iter()
                .flatten()
        })
        .collect()
}

fn triangle(n: usize) -> usize {
    (n * (n + 1)) / 2
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
        assert_eq!(answer, 2858);
    }
}
