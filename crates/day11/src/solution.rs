use std::collections::HashMap;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);

    let mut start = parsed;

    for _ in 0..25 {
        let evolved = evolve(start);
        start = evolved;
    }

    calculate_len(start)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);

    let mut start = parsed;

    for _ in 0..75 {
        let evolved = evolve(start);
        start = evolved;
    }

    calculate_len(start)
}

pub fn calculate_len(state: HashMap<usize, usize>) -> usize {
    state.iter().map(|(_, count)| count).sum()
}

pub fn evolve(state: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_state = HashMap::new();
    for (elem, count) in state {
        if elem == 0 {
            new_state
                .entry(1)
                .and_modify(|c| *c += count)
                .or_insert(count);
        } else {
            let elem_string = elem.to_string();
            if elem_string.len() % 2 == 0 {
                let (left_half, right_half) = elem_string.split_at(elem_string.len() / 2);
                new_state
                    .entry(left_half.parse().unwrap())
                    .and_modify(|c| *c += count)
                    .or_insert(count);
                new_state
                    .entry(right_half.parse().unwrap())
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                new_state
                    .entry(elem * 2024)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
    }
    new_state
}

fn parse(strings: &[String]) -> HashMap<usize, usize> {
    let mut state = HashMap::new();
    for elem in strings[0].split(' ') {
        state
            .entry(elem.parse().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "125 17".lines().map(|s| String::from(s.trim())).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(55312, part1(&get_input()));
    }
}
