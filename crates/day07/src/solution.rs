pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    let mut tot = 0;
    for (total, values) in parsed {
        let verified = verify_eq(total, &values);
        if verified {
            tot += total
        }
    }
    tot as usize
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    let mut tot = 0;
    for (total, values) in parsed {
        let verified = verify_eq_2(total, &values);
        if verified {
            tot += total
        }
    }
    tot as usize
}

pub fn verify_eq(total: u64, values: &Vec<u64>) -> bool {
    if values.len() == 2 {
        return total == values[0] + values[1] || total == values[0] * values[1];
    }

    let mut values_copy = values.clone();
    values_copy.drain(0..2);
    values_copy.insert(0, values[0] + values[1]);
    let add = verify_eq(total, &values_copy);

    if add {
        return true;
    }

    let mut values_copy = values.clone();
    values_copy.drain(0..2);
    values_copy.insert(0, values[0] * values[1]);
    let mult = verify_eq(total, &values_copy);

    if mult {
        return true;
    }

    false
}

pub fn verify_eq_2(total: u64, values: &Vec<u64>) -> bool {
    if values.len() == 2 {
        let string_val = (values[0].to_string() + &values[1].to_string())
            .parse()
            .unwrap();
        return total == values[0] + values[1]
            || total == values[0] * values[1]
            || total == string_val;
    }

    let mut values_copy = values.clone();
    values_copy.drain(0..2);
    values_copy.insert(0, values[0] + values[1]);
    let add = verify_eq_2(total, &values_copy);

    if add {
        return true;
    }

    let mut values_copy = values.clone();
    values_copy.drain(0..2);
    values_copy.insert(0, values[0] * values[1]);
    let mult = verify_eq_2(total, &values_copy);

    if mult {
        return true;
    }

    let mut values_copy = values.clone();
    values_copy.drain(0..2);
    let string_val = values[0].to_string() + &values[1].to_string();
    values_copy.insert(0, string_val.parse().unwrap());
    let concat = verify_eq_2(total, &values_copy);

    if concat {
        return true;
    }
    false
}

fn parse(strings: &[String]) -> Vec<(u64, Vec<u64>)> {
    let mut parsed = vec![];
    for line in strings {
        let mut splitter = line.split(": ");
        let num = splitter.next().unwrap().parse().unwrap();
        let rest = splitter
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        parsed.push((num, rest))
    }
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(3749, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(11387, part2(&get_input()));
    }
}
