use std::u8;

pub fn part1(input: &[String]) -> String {
    let parsed = parse(input);
    let out = execute_program(parsed.0, parsed.1, parsed.2, &parsed.3);
    output_to_string(out)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    find_copy_program(parsed.1, parsed.2, parsed.3)
}

fn find_copy_program(reg_b: usize, reg_c: usize, instructions: Vec<u8>) -> usize {
    let mut reg_a = 0;

    for i in 0..instructions.len() {
        let out_part: Vec<u8> = instructions
            .iter()
            .rev()
            .take(i + 1)
            .rev()
            .map(|x| *x)
            .collect();
        let mut new_reg_a = reg_a * 8;
        loop {
            let out = execute_program(new_reg_a, reg_b, reg_c, &instructions);
            if out == out_part {
                reg_a = new_reg_a;
                break;
            }
            new_reg_a += 1
        }
    }
    reg_a
}

fn execute_program(reg_a: usize, reg_b: usize, reg_c: usize, instructions: &Vec<u8>) -> Vec<u8> {
    let mut regs = (reg_a, reg_b, reg_c);
    let mut ip = 0;
    let mut output = vec![];
    while ip < instructions.len() {
        let opcode = instructions[ip];
        let operand = instructions[ip + 1];
        let instruction = Instruction::parse(opcode);
        match instruction {
            Instruction::Adv => {
                // division with result in A
                let numerator = regs.0;
                let denominator = get_combo_op(operand, &regs);
                regs.0 = numerator / 2_usize.pow(denominator.try_into().unwrap());
            }
            Instruction::Bxl => {
                let op1 = regs.1;
                let op2 = operand as usize;
                regs.1 = op1 ^ op2;
            }
            Instruction::Bst => {
                let op = get_combo_op(operand, &regs) % 8;
                regs.1 = op;
            }
            Instruction::Jnz => {
                if regs.0 != 0 {
                    ip = operand as usize;
                    continue;
                }
            }
            Instruction::Bxc => {
                let op1 = regs.1;
                let op2 = regs.2;
                regs.1 = op1 ^ op2;
            }
            Instruction::Out => {
                let value_to_print = get_combo_op(operand, &regs) % 8;
                output.push(value_to_print as u8);
            }
            Instruction::Bdv => {
                // division with result in B
                let numerator = regs.0;
                let denominator = get_combo_op(operand, &regs);
                regs.1 = numerator / 2_usize.pow(denominator.try_into().unwrap());
            }
            Instruction::Cdv => {
                // division with result in C
                let numerator = regs.0;
                let denominator = get_combo_op(operand, &regs);
                regs.2 = numerator / 2_usize.pow(denominator.try_into().unwrap());
            }
        }
        ip += 2;
    }
    output
}

fn output_to_string(output: Vec<u8>) -> String {
    let mut out_string = "".to_string();
    for (i, elem) in output.iter().enumerate() {
        if i == output.len() - 1 {
            out_string.push_str(&elem.to_string());
        } else {
            out_string.push_str(&format!("{},", elem));
        }
    }
    out_string
}

fn get_combo_op(operand: u8, regs: &(usize, usize, usize)) -> usize {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => regs.0,
        5 => regs.1,
        6 => regs.2,
        7 => unreachable!("Reserved Operand for comboop"),
        _ => unreachable!("Unrecognized operand for comboop"),
    }
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}
impl Instruction {
    fn parse(op: u8) -> Instruction {
        match op {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => unreachable!("Unrecognized opcode"),
        }
    }
}

fn parse(strings: &[String]) -> (usize, usize, usize, Vec<u8>) {
    let reg_a = strings[0]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let reg_b = strings[1]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let reg_c = strings[2]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let instructions = strings[4]
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();
    (reg_a, reg_b, reg_c, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_2() -> Vec<String> {
        "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part_instructions() {
        assert_eq!(vec![2], execute_program(10, 0, 0, &[0, 2, 5, 4].to_vec()));
        assert_eq!(vec![6], execute_program(0, 5, 0, &[1, 3, 5, 5].to_vec()));
        assert_eq!(vec![2], execute_program(0, 0, 10, &[2, 6, 5, 5].to_vec()));
        assert_eq!(vec![1], execute_program(0, 0, 9, &[2, 6, 5, 5].to_vec()));
        assert_eq!(
            vec![0, 1, 2],
            execute_program(10, 0, 0, &[5, 0, 5, 1, 5, 4].to_vec())
        );
        assert_eq!(
            vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0],
            execute_program(2024, 0, 0, &[0, 1, 5, 4, 3, 0].to_vec())
        );
        assert_eq!(vec![2], execute_program(0, 29, 0, &[1, 7, 5, 5].to_vec()));
        assert_eq!(
            vec![2],
            execute_program(0, 2024, 43690, &[4, 0, 5, 5].to_vec())
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!("4,6,3,5,6,3,5,2,1,0".to_string(), part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(117440, part2(&get_input_2()));
    }
}
