use std::collections::{HashMap, VecDeque};

// looked up some help

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    evaluate(&parsed.0, &parsed.1)
}

pub fn part2(input: &[String]) -> String {
    let parsed = parse(input);
    find_faulty_wires(&parsed.1, 45)
}

fn find_faulty_wires(ops: &Vec<OP>, bits: usize) -> String {
    let mut faulty = vec![];
    let last_z = "z".to_string() + &bits.to_string();
    for op in ops {
        match op {
            OP::AND { dst, .. } => {
                if dst.starts_with('z') && dst != &last_z {
                    faulty.push(op);
                }
            }
            OP::OR { dst, .. } => {
                if dst.starts_with('z') && dst != &last_z {
                    faulty.push(op);
                }
            }
            OP::XOR { op1, op2, dst } => {
                if !dst.starts_with('z') {
                    if !(op1.starts_with("x") || op1.starts_with("y"))
                        && !(op2.starts_with("x") || op2.starts_with("y"))
                    {
                        faulty.push(op);
                    }
                }
            }
        }
    }
    let mut faulty_2 = vec![];
    for op in ops {
        match op {
            OP::AND { op1, dst, .. } => {
                if op1 != "x00" {
                    let mut valid = false;
                    for op_2 in ops {
                        match op_2 {
                            OP::OR { op1, op2, .. } => {
                                if dst == op1 || dst == op2 {
                                    valid = true;
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                    if !valid {
                        faulty_2.push(op);
                    }
                }
            }
            OP::XOR { op1, op2, dst } => {
                if op1.starts_with("x") && op2.starts_with("y") && op1 != "x00" {
                    let mut valid = false;
                    for op_2 in ops {
                        match op_2 {
                            OP::XOR {
                                op1: op1_2,
                                op2: op2_2,
                                ..
                            } => {
                                if dst == op1_2 || dst == op2_2 {
                                    valid = true;
                                    break;
                                }
                            }
                            _ => (),
                        }
                    }
                    if !valid {
                        faulty_2.push(op);
                    }
                }
            }
            _ => (),
        }
    }
    for elem in faulty_2 {
        if !faulty.contains(&elem) {
            faulty.push(elem);
        }
    }
    let mut faulty_str: Vec<String> = faulty.iter().map(|op| op.get_dst()).collect();
    faulty_str.sort();
    let mut s = "".to_string();
    for (i, faulty) in faulty_str.iter().enumerate() {
        if i == faulty_str.len() - 1 {
            s.push_str(&format!("{}", faulty));
        } else {
            s.push_str(&format!("{},", faulty));
        }
    }
    s
}

fn evaluate(memory: &HashMap<String, u8>, ops: &Vec<OP>) -> usize {
    let mut memory = memory.clone();
    let mut ops: VecDeque<&OP> = ops.iter().map(|x| x).collect();
    while !ops.is_empty() {
        let op = ops.pop_front().unwrap();
        match op {
            OP::AND { op1, op2, dst } => {
                if let Some(op1_val) = memory.get(op1) {
                    if let Some(op2_val) = memory.get(op2) {
                        memory.insert(dst.clone(), op1_val & op2_val);
                        continue;
                    }
                }
            }
            OP::OR { op1, op2, dst } => {
                if let Some(op1_val) = memory.get(op1) {
                    if let Some(op2_val) = memory.get(op2) {
                        memory.insert(dst.clone(), op1_val | op2_val);
                        continue;
                    }
                }
            }
            OP::XOR { op1, op2, dst } => {
                if let Some(op1_val) = memory.get(op1) {
                    if let Some(op2_val) = memory.get(op2) {
                        memory.insert(dst.clone(), op1_val ^ op2_val);
                        continue;
                    }
                }
            }
        }
        ops.push_back(op);
    }
    extract_value_from_memory(&memory)
}

fn extract_value_from_memory(memory: &HashMap<String, u8>) -> usize {
    let mut memory_vec: Vec<(&String, &u8)> =
        memory.iter().filter(|x| x.0.starts_with("z")).collect();
    memory_vec.sort_by(|a, b| b.0.cmp(a.0));
    let mut s = "".to_string();
    for (_, val) in memory_vec {
        s.push_str(&val.to_string());
    }
    usize::from_str_radix(&s, 2).unwrap()
}

fn parse(strings: &[String]) -> (HashMap<String, u8>, Vec<OP>) {
    let mut is_first_part = true;
    let mut initial_values = HashMap::new();
    let mut ops = vec![];
    for line in strings {
        if line == "" {
            is_first_part = false;
            continue;
        }
        if is_first_part {
            let mut splitter = line.split(": ");
            let variable_name = splitter.next().unwrap();
            let variable_value = splitter.next().unwrap().parse().unwrap();
            initial_values.insert(variable_name.to_string(), variable_value);
        } else {
            let mut splitter = line.split(" ");
            let op1 = splitter.next().unwrap().to_string();
            let op = splitter.next().unwrap();
            let op2 = splitter.next().unwrap().to_string();
            let dst = splitter.skip(1).next().unwrap().to_string();
            let full_op = match op {
                "AND" => OP::AND { op1, op2, dst },
                "OR" => OP::OR { op1, op2, dst },
                "XOR" => OP::XOR { op1, op2, dst },
                _ => unreachable!("Unexpected operation in input"),
            };
            ops.push(full_op);
        }
    }
    (initial_values, ops)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OP {
    AND {
        op1: String,
        op2: String,
        dst: String,
    },
    OR {
        op1: String,
        op2: String,
        dst: String,
    },
    XOR {
        op1: String,
        op2: String,
        dst: String,
    },
}
impl OP {
    fn get_dst(&self) -> String {
        match self {
            OP::AND { dst, .. } => dst.clone(),
            OP::OR { dst, .. } => dst.clone(),
            OP::XOR { dst, .. } => dst.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_small() -> Vec<String> {
        "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input() -> Vec<String> {
        "x00: 1
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
tnw OR pbm -> gnj"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1_small() {
        assert_eq!(4, part1(&get_input_small()));
    }
    #[test]
    fn test_part1() {
        assert_eq!(2024, part1(&get_input()));
    }
}
