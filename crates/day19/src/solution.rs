use std::collections::HashMap;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    count_doable_patterns(&parsed.0, &parsed.1)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    count_way_patterns(&parsed.0, &parsed.1)
}

pub fn count_doable_patterns(
    available_towels: &Vec<Vec<char>>,
    patterns: &Vec<Vec<char>>,
) -> usize {
    let mut count = 0;
    let mut cache = HashMap::new();
    for pattern in patterns.iter() {
        if verify_pattern(available_towels, pattern, &mut cache) {
            count += 1;
        }
    }
    count
}

pub fn count_way_patterns(available_towels: &Vec<Vec<char>>, patterns: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let mut cache = HashMap::new();
    for pattern in patterns.iter() {
        count += count_pattern(available_towels, pattern, &mut cache);
    }
    count
}

fn verify_pattern(
    available_towels: &Vec<Vec<char>>,
    pattern: &Vec<char>,
    cache: &mut HashMap<Vec<char>, bool>,
) -> bool {
    if let Some(value) = cache.get(pattern) {
        return *value;
    }
    if pattern.is_empty() {
        return true;
    }
    for towel in available_towels {
        if pattern.starts_with(&towel) {
            let (_, remaining_pattern) = pattern.split_at(towel.len());
            if verify_pattern(available_towels, &remaining_pattern.to_vec(), cache) {
                cache.insert(pattern.to_vec(), true);
                return true;
            }
        }
    }
    cache.insert(pattern.to_vec(), false);
    false
}

fn count_pattern(
    available_towels: &Vec<Vec<char>>,
    pattern: &Vec<char>,
    cache: &mut HashMap<Vec<char>, usize>,
) -> usize {
    if let Some(value) = cache.get(pattern) {
        return *value;
    }
    if pattern.is_empty() {
        return 1;
    }
    let mut count = 0;
    let mut sorted_towels = available_towels.clone();
    sorted_towels.sort_by(|x, y| y.len().cmp(&x.len()));
    for towel in sorted_towels {
        if pattern.starts_with(&towel) {
            let (_, remaining_pattern) = pattern.split_at(towel.len());
            count += count_pattern(available_towels, &remaining_pattern.to_vec(), cache);
        }
    }
    cache.insert(pattern.to_vec(), count);
    count
}

fn parse(strings: &[String]) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut first = true;
    let mut available_towels: Vec<Vec<char>> = vec![];
    let mut patterns: Vec<Vec<char>> = vec![];
    for line in strings {
        if line.is_empty() {
            first = false;
            continue;
        }

        if first {
            // parse list of available towels
            available_towels = line.split(", ").map(|s| s.chars().collect()).collect()
        } else {
            // parse the patters to create
            patterns.push(line.chars().collect());
        }
    }
    (available_towels, patterns)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(6, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(16, part2(&get_input()));
    }
}
