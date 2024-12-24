use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    parsed
        .iter()
        .map(|secret| nth_secret_number(2000, *secret))
        .sum()
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    find_best_sequence(parsed)
}

fn find_best_sequence(secrets: Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for secret in secrets {
        let prices = get_prices(2000 + 1, secret);
        let changes: Vec<i8> = prices
            .windows(2)
            .map(|x| x[1] as isize - x[0] as isize)
            .map(|x| x as i8)
            .collect();
        let mut seen: HashSet<Vec<i8>> = HashSet::new();
        for (i, subseq) in changes.windows(4).enumerate() {
            if seen.contains(&subseq.to_vec()) {
                continue;
            }
            map.entry(subseq.to_vec())
                .and_modify(|x| *x += prices[i + 4])
                .or_insert(prices[i + 4]);
            seen.insert(subseq.to_vec());
        }
    }

    *map.values().max().unwrap()
}

fn get_prices(n: usize, secret_number: usize) -> Vec<usize> {
    let mut prices = vec![];
    let mut secret_number = secret_number;
    for _ in 0..n {
        prices.push(secret_number % 10);
        secret_number = next_secret_number(secret_number)
    }
    prices
}

fn nth_secret_number(n: usize, secret_number: usize) -> usize {
    let mut secret_number = secret_number;
    for _ in 0..n {
        secret_number = next_secret_number(secret_number)
    }
    secret_number
}

fn next_secret_number(secret_number: usize) -> usize {
    let step1 = prune(mix(secret_number * 64, secret_number));
    let step2 = prune(mix(step1 / 32, step1));
    let step3 = prune(mix(step2 * 2048, step2));
    step3
}

fn mix(value: usize, secret_number: usize) -> usize {
    value ^ secret_number
}
fn prune(secret_number: usize) -> usize {
    secret_number % 16777216
}

fn parse(strings: &[String]) -> Vec<usize> {
    strings.iter().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "1
10
100
2024"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_2() -> Vec<String> {
        "1
2
3
2024"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(37327623, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(23, part2(&get_input_2()));
    }
}
