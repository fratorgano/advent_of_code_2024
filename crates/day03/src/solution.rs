use regex::Regex;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    multiply(parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse_2(input);
    multiply(parsed)
}

pub fn multiply(numbers: Vec<(usize, usize)>) -> usize {
    let mut total = 0;
    for (num1, num2) in numbers {
        total += num1 * num2
    }
    total
}

fn parse(strings: &[String]) -> Vec<(usize, usize)> {
    let mut ops = vec![];
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)").unwrap();
    for line in strings {
        let caps = re.captures_iter(line);
        for m in caps {
            let op1 = m["first"].parse().unwrap();
            let op2 = m["second"].parse().unwrap();
            ops.push((op1, op2));
        }
    }
    ops
}

fn parse_2(strings: &[String]) -> Vec<(usize, usize)> {
    let mut ops = vec![];
    let re = Regex::new(r"mul\((?<first>\d+),(?<second>\d+)\)|(?<enable>do(?:n't)?\(\))").unwrap();
    let mut is_enabled = true;
    for line in strings {
        let caps = re.captures_iter(line);
        for m in caps {
            if m.get(3).is_some() {
                is_enabled = m["enable"] == *"do()";
            } else if is_enabled {
                let op1 = m["first"].parse().unwrap();
                let op2 = m["second"].parse().unwrap();
                ops.push((op1, op2));
            }
        }
    }
    ops
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(&get_input()));
    }
}
