use std::collections::HashMap;

use crate::utils::answers::Answer;

#[derive(Debug, PartialEq, Eq, Clone)]
struct LogicGraph {
    gates: Vec<Gate>,
    lines: HashMap<[u8; 3], usize>,
}

impl LogicGraph {
    fn new() -> Self {
        Self {
            gates: Vec::new(),
            lines: HashMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Operation {
    And,
    Or,
    Xor,
    Nop,
}

impl Operation {
    fn calculate(&self, mut left: bool, mut right: bool) -> Option<bool> {
        Some(match self {
            Operation::And => left && right,
            Operation::Or => left || right,
            Operation::Xor => left != right,
            Operation::Nop => return None,
        })
    }
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            _ => return Err(format!("'{value}' not a valid boolean operation")),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Gate {
    operation: Operation,
    inputs: Option<[[u8; 3]; 2]>,
    value: Option<bool>,
    out: [u8; 3],
}

impl Gate {
    fn value(&self, graph: &mut LogicGraph) -> bool {
        if let Some(value) = self.value {
            return value;
        }

        let [left, right] = self.inputs.expect("either value or inputs should be set");
        let left = graph.gates[graph.lines[&left]];
        let right = graph.gates[graph.lines[&right]];
        self.operation
            .calculate(left.value(graph), right.value(graph))
            .unwrap()
    }
}

pub fn part1(input: &str) -> Answer {
    let mut graph = parse(input);
    let mut answer = 0;

    let mut key = [b'z', 0, 0];
    for i in 0.. {
        key[1] = i / 10 + b'0';
        key[2] = i % 10 + b'0';

        let Some(&line) = graph.lines.get(&key) else {
            break;
        };
        let gate = graph.gates[line];
        let value = gate.value(&mut graph) as u64;
        answer = (value << i) | answer;
    }
    answer.into()
}

pub fn check_default_result_binary(input: &str) {
    let mut graph = parse(input);
    let mut answer = 0;

    let mut key = [b'z', 0, 0];
    for i in 0.. {
        key[1] = i / 10 + b'0';
        key[2] = i % 10 + b'0';

        let Some(&line) = graph.lines.get(&key) else {
            break;
        };
        let gate = graph.gates[line];
        let value = gate.value(&mut graph) as u64;
        answer = (value << i) | answer;
    }
    key = [b'x', 0, 0];
    let mut a = 0;
    for i in 0.. {
        key[1] = i / 10 + b'0';
        key[2] = i % 10 + b'0';

        let Some(&line) = graph.lines.get(&key) else {
            break;
        };
        let gate = graph.gates[line];
        let value = gate.value(&mut graph) as u64;
        a = (value << i) | a;
    }

    key = [b'y', 0, 0];
    let mut b = 0;
    for i in 0.. {
        key[1] = i / 10 + b'0';
        key[2] = i % 10 + b'0';

        let Some(&line) = graph.lines.get(&key) else {
            break;
        };
        let gate = graph.gates[line];
        let value = gate.value(&mut graph) as u64;
        b = (value << i) | b;
    }

    let expect = a + b;
    println!("{a:046b}");
    println!("{b:046b}");
    println!("{answer:046b}");
    println!("{expect:046b}");
}

fn find_rule(
    graph: &mut LogicGraph,
    wire1: [u8; 3],
    wire2: [u8; 3],
    operation: Operation,
) -> Option<(usize, Gate)> {
    for (i, &gate) in graph.gates.iter().enumerate() {
        let Some(inputs) = gate.inputs else {
            continue;
        };
        if gate.operation == operation && inputs.contains(&wire1) && inputs.contains(&wire2) {
            return Some((i, gate));
        }
    }
    None
}

fn swap(
    graph: &mut LogicGraph,
    wire1: [u8; 3],
    wire2: [u8; 3],
    operation: Operation,
    swaps: &mut Vec<[u8; 3]>,
) {
    swaps.extend([wire1, wire2].iter().copied());
    let wire1 = graph.lines[&wire1];
    let wire2 = graph.lines[&wire2];
    (graph.gates[wire1], graph.gates[wire2]) = (graph.gates[wire2], graph.gates[wire1]);
}

pub fn part2(input: &str) -> Answer {
    let mut graph = parse(input);

    let mut gate_and = vec![None; 46];
    let mut gate_xor = vec![None; 46];
    let mut gate_z = vec![None; 46];
    let mut gate_tmp = vec![None; 46];
    let mut gate_carry = vec![None; 46];

    let mut swaps = Vec::new();

    let mut i = 0;
    let mut x = [b'x', b'0', b'0'];
    let mut y = [b'y', b'0', b'0'];
    let mut z = [b'z', b'0', b'0'];

    gate_and[i] = find_rule(&mut graph, x, y, Operation::And);
    gate_xor[i] = find_rule(&mut graph, x, y, Operation::Xor);
    gate_z[i] = gate_xor[i];
    gate_carry[i] = gate_and[i];

    for i in 1..46 {
        (x[1], x[2]) = (i as u8 / 10 + b'0', i as u8 % 10 + b'0');
        (y[1], y[2]) = (i as u8 / 10 + b'0', i as u8 % 10 + b'0');
        (z[1], z[2]) = (i as u8 / 10 + b'0', i as u8 % 10 + b'0');
        let mut check = true;
        while check {
            check = false;

            gate_and[i] = find_rule(&mut graph, x, y, Operation::And);
            gate_xor[i] = find_rule(&mut graph, x, y, Operation::Xor);
            let gate = graph.lines[&z];
            let gate = graph.gates[gate];

            let Some([left, right]) = gate.inputs else {
                continue;
            };

            if let Some((carry_gate_index, carry_gate)) = gate_carry[i - 1] {
                if left == carry_gate.out
                    && (gate_xor[i].is_none() || right != gate_xor[i].unwrap().1.out)
                {
                    let operation = gate.operation;
                    swap(&mut graph, left, right, operation, &mut swaps);
                    check = true;
                    continue;
                }
                if right == carry_gate.out
                    && (gate_xor[i].is_none() || left != gate_xor[i].unwrap().1.out)
                {
                    let operation = gate.operation;
                    swap(&mut graph, left, right, operation, &mut swaps);
                    check = true;
                    continue;
                }
            }

            match (gate_xor[i], gate_carry[i - 1], gate_and[i]) {
                (
                    Some((xor_gate_index, xor_gate)),
                    Some((carry_gate_index, carry_gate)),
                    Some((and_gate_index, and_gate)),
                ) => {
                    gate_z[i] = find_rule(&mut graph, xor_gate.out, carry_gate.out, Operation::Xor);
                    if gate_z[i].unwrap().1.out != z {
                        swap(
                            &mut graph,
                            gate_z[i].unwrap().1.out,
                            z,
                            Operation::Xor,
                            &mut swaps,
                        );
                        check = true;
                        continue;
                    }
                    gate_tmp[i] =
                        find_rule(&mut graph, xor_gate.out, carry_gate.out, Operation::And);
                    gate_carry[i] = find_rule(
                        &mut graph,
                        gate_tmp[i].unwrap().1.out,
                        and_gate.out,
                        Operation::Or,
                    );
                }
                _ => {}
            }
        }
    }

    Answer::InProgress
}

fn to_arr<const N: usize>(input: &str) -> [u8; N] {
    let mut array = [0; N];
    for (i, c) in input.bytes().enumerate() {
        array[i] = c;
    }
    array
}

fn parse(input: &str) -> LogicGraph {
    let mut graph = LogicGraph::new();

    let (prefix, suffix) = input.split_once("\n\n").unwrap();

    for line in prefix.lines() {
        let name: [u8; 3] = to_arr::<3>(&line[0..3]);
        let value = Some(if line.bytes().nth(5).unwrap() == b'0' {
            false
        } else {
            true
        });
        graph.lines.insert(name, graph.gates.len());
        graph.gates.push(Gate {
            operation: Operation::Nop,
            inputs: None,
            value,
            out: name,
        });
    }

    for line in suffix.lines() {
        // println!("{line}");
        let split = line.split_whitespace().collect::<Vec<_>>();
        let left = to_arr::<3>(split[0]);
        let operation = Operation::try_from(split[1]).unwrap();
        let right = to_arr::<3>(split[2]);
        let name = to_arr::<3>(split[4]);

        let value = None;
        graph.lines.insert(name, graph.gates.len());
        graph.gates.push(Gate {
            operation,
            inputs: Some([left, right]),
            value,
            out: name,
        });
    }

    graph
}
