use std::{
    collections::{HashMap, HashSet},
    fs::{self},
    time::Instant,
};

#[derive(Debug)]
struct Input {
    edges: Vec<(String, String)>,
}

fn parse_input(s: &str) -> Input {
    let edges = s
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_owned(), b.to_owned())
        })
        .collect::<Vec<_>>();
    Input { edges }
}

fn solve(input: &Input) -> usize {
    let mut adjacency_matrix: HashMap<String, HashSet<String>> = HashMap::new();
    input
        .edges
        .iter()
        .flat_map(|(a, b)| [(a, b), (b, a)])
        .for_each(|(a, b)| {
            adjacency_matrix
                .entry(a.to_owned())
                .and_modify(|s| {
                    s.insert(b.to_owned());
                })
                .or_insert_with(|| HashSet::from([b.to_owned()]));
        });

    adjacency_matrix
        .iter()
        .flat_map(|(k, vs)| {
            let vs_vec: Vec<_> = vs.iter().collect();
            iter_pairs(&vs_vec)
                .filter(|(a, b)| is_connected(&adjacency_matrix, a, b))
                .map(|(a, b)| {
                    let mut x = [k, a, b];
                    x.sort();
                    x
                })
                .collect::<Vec<_>>()
        })
        .filter(|s| s.iter().any(|c| c.starts_with('t')))
        .map(|s| s.map(|s| s.to_owned()))
        .collect::<HashSet<_>>()
        .len()
}

fn is_connected(adjacency_matrix: &HashMap<String, HashSet<String>>, a: &str, b: &str) -> bool {
    adjacency_matrix.get(a).is_some_and(|s| s.contains(b))
}

/// Iterates forward pairs
/// i.e. for the slice [a,b,c], yields (a,b), (a,c), (b,c)
fn iter_pairs<T>(slice: &[T]) -> impl Iterator<Item = (&T, &T)> {
    slice
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| slice[(i + 1)..].iter().map(move |p2| (p1, p2)))
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day23/input").unwrap();
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
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 7);
    }
}
