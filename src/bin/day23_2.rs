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

fn solve(input: &Input) -> String {
    // This problem I think is "largest complete subgraph".
    // I was never that hot on graph algorithms, I'll avoid looking this one up
    // and see what I can come up with by myself.

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

    // The intention here is to generate every complete subgraph,
    // then iterate through and find the largest one.
    //
    // Honestly not sure if this algorithm is actually guaranteed
    // to find all complete subgraphs or if I just got lucky.
    //
    // Idea: for every vertex v, start with a set S (v,v1) from its first edge.
    // Then for every other edge (v, vn) from this vertex,
    // add vn to S IF the set of vertices connected to vn is a superset of S.
    adjacency_matrix
        .iter()
        .map(|(k, vs)| {
            let vs_vec: Vec<_> = vs.iter().collect();
            let mut complete_vs = HashSet::from([k.to_owned(), vs_vec[0].to_owned()]);

            vs_vec[1..].iter().for_each(|&v| {
                let connections = adjacency_matrix.get(v).unwrap();
                if connections.is_superset(&complete_vs) {
                    complete_vs.insert(v.to_owned());
                }
            });
            complete_vs
        })
        .max_by_key(|s| s.len())
        .map(|s| {
            let mut names = s.into_iter().collect::<Vec<_>>();
            names.sort();
            names.join(",")
        })
        .unwrap()
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

        assert_eq!(answer, "co,de,ka,ta");
    }
}
