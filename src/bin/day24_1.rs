use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::{self},
    time::Instant,
};

#[derive(Debug, Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone)]
struct Gate {
    left: String,
    op: GateType,
    right: String,
}

#[derive(Debug)]
struct Input {
    wire_vals: Vec<(String, bool)>,
    gates: Vec<(Gate, String)>,
}

lazy_static! {
    static ref WIRE_REGEX: Regex = Regex::new("^([0-9a-z]+): (0|1)$").unwrap();
    static ref GATE_REGEX: Regex =
        Regex::new("^([0-9a-z]+) (AND|OR|XOR) ([0-9a-z]+) -> ([0-9a-z]+)$").unwrap();
}

fn parse_input(s: &str) -> Input {
    let mut lines = s.lines();

    let wires = (&mut lines)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let captures = WIRE_REGEX.captures(l).unwrap();
            (captures[1].to_owned(), parse_bool(&captures[2]))
        })
        .collect();

    let gates = lines
        .map(|l| {
            let captures = GATE_REGEX.captures(l).unwrap();
            (
                Gate {
                    left: captures[1].to_owned(),
                    op: parse_op(&captures[2]),
                    right: captures[3].to_owned(),
                },
                captures[4].to_owned(),
            )
        })
        .collect();

    Input {
        wire_vals: wires,
        gates,
    }
}

fn parse_bool(v: &str) -> bool {
    match v {
        "1" => true,
        "0" => false,
        _ => panic!("invalid bool {}", v),
    }
}

fn parse_op(op: &str) -> GateType {
    match op {
        "AND" => GateType::And,
        "OR" => GateType::Or,
        "XOR" => GateType::Xor,
        _ => panic!("invalid gate op {}", op),
    }
}

#[derive(Debug, Clone)]
enum ValueEntry {
    Literal(bool),
    Computed(Gate),
}

fn solve(input: &Input) -> usize {
    let mut values_cache: HashMap<_, _> = input
        .wire_vals
        .iter()
        .map(|w| (w.0.clone(), ValueEntry::Literal(w.1)))
        .chain(
            input
                .gates
                .iter()
                .map(|g| (g.1.clone(), ValueEntry::Computed(g.0.clone()))),
        )
        .collect();

    let mut z_wires = values_cache
        .keys()
        .cloned()
        .filter(|w| w.starts_with('z'))
        .collect::<Vec<_>>();
    z_wires.sort();

    z_wires
        .iter()
        .rev()
        .map(|w| eval_wire(&mut values_cache, w))
        .fold(0usize, |acc, v| (acc << 1) + (v as usize))
}

fn eval_wire(values_cache: &mut HashMap<String, ValueEntry>, w: &str) -> bool {
    let val = values_cache[w].clone();
    match val {
        ValueEntry::Literal(v) => v,
        ValueEntry::Computed(gate) => {
            let left = eval_wire(values_cache, &gate.left);
            let right = eval_wire(values_cache, &gate.right);
            let val = do_op(left, right, gate.op);
            values_cache.insert(w.to_owned(), ValueEntry::Literal(val));
            val
        }
    }
}

fn do_op(left: bool, right: bool, op: GateType) -> bool {
    match op {
        GateType::And => left && right,
        GateType::Or => left || right,
        GateType::Xor => left != right,
    }
}

fn main() {
    let start_time = Instant::now();
    let input_str = fs::read_to_string("data/day24/input").unwrap();
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
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";
        let input = parse_input(&input_str);
        let answer = solve(&input);

        assert_eq!(answer, 2024);
    }
}
