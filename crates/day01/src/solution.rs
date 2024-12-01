use std::collections::HashMap;

pub fn part1(input: &[String]) -> usize {
    let mut parsed = parse(input);
    parsed.0.sort();
    parsed.1.sort();
    calculate_difference(parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    let occurencies_list2 = count_occurencies(&parsed.1);
    calculate_similarity_score(parsed.0, &occurencies_list2)
}

fn calculate_similarity_score(list: Vec<usize>, occurrencies: &HashMap<usize, usize>) -> usize {
    list.iter()
        .map(|x| occurrencies.get(x).unwrap_or(&0) * x)
        .sum()
}

fn count_occurencies(list: &Vec<usize>) -> HashMap<usize, usize> {
    let mut occurencies = HashMap::new();
    for num in list {
        occurencies.entry(*num).and_modify(|v| *v += 1).or_insert(1);
    }
    occurencies
}

fn calculate_difference(vecs: (Vec<usize>, Vec<usize>)) -> usize {
    let mut diff = 0;
    for (num1, num2) in vecs.0.iter().zip(vecs.1.iter()) {
        diff += num1.abs_diff(*num2)
    }
    diff
}

fn parse(strings: &[String]) -> (Vec<usize>, Vec<usize>) {
    let mut list1: Vec<usize> = vec![];
    let mut list2: Vec<usize> = vec![];

    for l in strings {
        let mut split_line = l.split("   ");
        let num1 = split_line.next().unwrap().parse().unwrap();
        let num2 = split_line.next().unwrap().parse().unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    (list1, list2)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "3   4
4   3
2   5
1   3
3   9
3   3"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(11, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(31, part2(&get_input()));
    }
}
