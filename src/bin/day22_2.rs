use std::{
    collections::HashMap,
    fs::{self},
    iter,
    time::Instant,
};

#[derive(Debug)]
struct Input {
    lines: Vec<u64>,
}

fn parse_input(s: &str) -> Input {
    let lines = s.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>();
    Input { lines }
}

fn solve(input: &Input) -> u64 {
    let mut master_lookup = HashMap::new();
    input
        .lines
        .iter()
        .map(|l| make_code_lookup(*l))
        .for_each(|m| {
            for (k, v) in m {
                master_lookup.entry(k).and_modify(|x| *x += v).or_insert(v);
            }
        });

    master_lookup.values().copied().max().unwrap()
}

type Code = (i64, i64, i64, i64);
fn next_code(c: Code, val: i64) -> Code {
    (c.1, c.2, c.3, val)
}

fn make_code_lookup(l: u64) -> HashMap<Code, u64> {
    let mut price_and_delta_it =
        iter_secret_nums(l)
            .skip(1)
            .take(2000)
            .map(|v| v % 10)
            .scan(l, |prev, curr| {
                let delta = (curr as i64) - (*prev as i64);
                *prev = curr;
                Some((curr, delta))
            });

    let p1 = price_and_delta_it.next().unwrap();
    let p2 = price_and_delta_it.next().unwrap();
    let p3 = price_and_delta_it.next().unwrap();

    let mut lookup = HashMap::new();
    price_and_delta_it
        .scan((0i64, p1.1, p2.1, p3.1), |prev_code, (price, delta)| {
            let code = next_code(*prev_code, delta);
            *prev_code = code;
            Some((code, price))
        })
        .for_each(|(k, v)| {
            lookup.entry(k).or_insert(v);
        });
    lookup
}

fn iter_secret_nums(l: u64) -> impl Iterator<Item = u64> {
    iter::successors(Some(l), |n| Some(next_num(*n)))
}

fn next_num(mut n: u64) -> u64 {
    n = prune(mix(n, n * 64));
    n = prune(mix(n, n / 32));
    n = prune(mix(n, n * 2048));
    n
}

fn mix(a: u64, b: u64) -> u64 {
    a ^ b
}

fn prune(n: u64) -> u64 {
    n % 16777216
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day22/input").unwrap();
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
1
2
3
2024
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 23);
    }

    #[test]
    fn test_iter_secret_nums() {
        let initial = 123;
        let v = iter_secret_nums(initial)
            .skip(1)
            .take(10)
            .collect::<Vec<_>>();
        assert_eq!(
            v,
            vec![
                15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
                5908254,
            ]
        )
    }
}
