use std::collections::HashMap;

pub fn part1(input: &[String]) -> usize {
    let (rules, updates) = parse(input);
    sum_middle_valid_updates(updates, rules)
}

pub fn part2(input: &[String]) -> usize {
    let (rules, updates) = parse(input);
    sum_middle_fixed_updates(updates, rules)
}

fn sum_middle_fixed_updates(updates: Vec<Vec<u8>>, rules: HashMap<u8, Vec<u8>>) -> usize {
    let mut total_fixed = 0;
    for update in updates {
        if !verify_rule(&update, &rules) {
            let fixed_update = fix_update(&update, &rules);
            total_fixed += fixed_update[fixed_update.len() / 2] as usize
        }
    }
    total_fixed
}

fn sum_middle_valid_updates(updates: Vec<Vec<u8>>, rules: HashMap<u8, Vec<u8>>) -> usize {
    let mut total_valid = 0;
    for update in updates {
        if verify_rule(&update, &rules) {
            total_valid += update[update.len() / 2] as usize
        }
    }
    total_valid
}

fn fix_update(update: &Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    let mut update = update.clone();
    let mut updated = true;
    while updated {
        let mut new_update = update.clone();
        updated = false;
        for (page_pos, page) in update.iter().enumerate() {
            let mut splitter = update.split(|p| p == page);
            let before = splitter.next().unwrap();
            for (i, elem) in before.iter().enumerate() {
                if let Some(rule_set) = rules.get(page) {
                    if rule_set.contains(elem) {
                        new_update.remove(page_pos);
                        new_update.insert(i, *page);
                        updated = true;
                        break;
                    }
                }
            }
            if updated {
                break;
            }
        }
        update = new_update;
    }
    update
}

fn verify_rule(update: &Vec<u8>, rules: &HashMap<u8, Vec<u8>>) -> bool {
    for page in update.iter() {
        let mut splitter = update.split(|p| p == page);
        let before = splitter.next().unwrap();
        for elem in before {
            if let Some(rule_set) = rules.get(page) {
                if rule_set.contains(elem) {
                    return false;
                }
            }
        }
    }
    true
}

fn parse(strings: &[String]) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let mut rules = HashMap::new();
    let mut updates = vec![];
    let mut parsing_rules = true;

    for line in strings {
        if line.trim().is_empty() {
            parsing_rules = false;
            continue;
        }
        if parsing_rules {
            let mut splitter = line.split('|');
            let key = splitter.next().unwrap().parse().unwrap();
            let value = splitter.next().unwrap().parse().unwrap();
            rules
                .entry(key)
                .and_modify(|vec: &mut Vec<u8>| vec.push(value))
                .or_insert(vec![value]);
        } else {
            let pages = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(pages);
        }
    }

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(143, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, part2(&get_input()));
    }
}
