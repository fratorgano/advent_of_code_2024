pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    count_safe(&parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    count_safe_2(&parsed)
}

pub fn count_safe(reports: &Vec<Vec<isize>>) -> usize {
    reports
        .iter()
        .map(|r| if is_safe(&r) { 1 } else { 0 })
        .sum()
}

pub fn count_safe_2(reports: &Vec<Vec<isize>>) -> usize {
    reports
        .iter()
        .map(|r| if is_safe_2(&r) { 1 } else { 0 })
        .sum()
}

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
pub fn is_safe(report: &Vec<isize>) -> bool {
    let mut is_increasing = None;
    for values in report.windows(2) {
        let prev = values[0];
        let next = values[1];

        if prev == next {
            return false;
        }

        let diff: isize = next - prev;

        if is_increasing.is_none() {
            if diff > 0 {
                if diff > 3 {
                    return false;
                }
                is_increasing = Some(true)
            }
            if diff < 0 {
                if diff < -3 {
                    return false;
                }
                is_increasing = Some(false)
            }
            continue;
        }
        if is_increasing.unwrap() {
            // increasing
            if diff < 0 || diff > 3 {
                return false;
            }
        } else {
            // decreasing
            if diff > 0 || diff < -3 {
                return false;
            }
        }
    }
    true
}

pub fn is_safe_2(report: &Vec<isize>) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut dampened_report = report.clone();
        dampened_report.remove(i);
        if is_safe(&dampened_report) {
            return true;
        }
    }
    false
}

fn parse(strings: &[String]) -> Vec<Vec<isize>> {
    let mut reports = vec![];
    for line in strings {
        let report = line.split(' ').map(|x| x.parse().unwrap()).collect();
        reports.push(report);
    }
    reports
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(&get_input()));
    }
}
