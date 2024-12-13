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
    let mut runs = to_runs(&input.line);
    compact(&mut runs);
    compute_checksum(&runs)
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

fn compute_checksum(runs: &[Run]) -> usize {
    runs.iter()
        .scan(0usize, |pos, r| {
            let start = *pos;
            *pos += usize::from(r.len);
            Some((start, r))
        })
        .filter_map(|(start, r)| {
            r.file_id.map(|file_id| {
                let tri = triangle(usize::from(r.len) - 1);
                (tri * file_id) + (start * usize::from(r.len) * file_id)
            })
        })
        .sum()
}

fn triangle(n: usize) -> usize {
    (n * (n + 1)) / 2
}

fn compact(runs: &mut Vec<Run>) {
    let mut end_idx = runs.len() - 1;

    while end_idx >= 1 {
        let end_run = &runs[end_idx];
        if end_run.file_id.is_none() {
            end_idx -= 1;
            continue;
        };

        let pos = runs
            .iter()
            .take(end_idx)
            .position(|r| r.file_id.is_none() && r.len >= end_run.len);

        if let Some(pos) = pos {
            assert!(pos < end_idx);
            let remaining_space = runs[pos].len - end_run.len;
            runs[pos] = *end_run;
            runs[end_idx].file_id = None;
            if remaining_space > 0 {
                runs.insert(pos + 1, Run::free(remaining_space));
                end_idx += 1;
            }
        }

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
    use std::iter;

    use super::*;

    fn format_runs(runs: &[Run]) -> String {
        runs.iter()
            .flat_map(|r| {
                iter::repeat_n(
                    r.file_id.map(|id| id.to_string()).unwrap_or(".".to_owned()),
                    r.len.into(),
                )
            })
            .collect()
    }

    #[test]
    fn test_compute_checksum() {
        let v = vec![Run::file(4, 3)];
        assert_eq!(compute_checksum(&v), 18);
    }

    #[test]
    fn test_solve1() {
        let input_str = "\
2333133121414131402
";
        let input = parse_input(&input_str);
        let mut runs = to_runs(&input.line);

        compact(&mut runs);
        assert_eq!(
            format_runs(&runs),
            "00992111777.44.333....5555.6666.....8888.."
        );

        let answer = solve(&input);
        assert_eq!(answer, 2858);
    }
}
