use regex::Regex;

pub fn part1(input: &String) -> usize {
    let parsed = parse(input);
    let mut tot_coin = 0;
    for machine in parsed {
        tot_coin += find_min_coin(machine)
    }
    tot_coin
}

pub fn part2(input: &String) -> usize {
    let parsed = parse(input);
    let mut tot_coin = 0;
    for machine in parsed {
        let updated_machine = update_machine(machine);
        tot_coin += find_min_coin(updated_machine)
    }
    tot_coin
}

pub fn find_min_coin(machine: ((isize, isize, isize), (isize, isize, isize))) -> usize {
    let intersection_opt = intersection_int(machine.0, machine.1);
    if let Some(intersection) = intersection_opt {
        return (intersection.0 * 3 + intersection.1 * 1)
            .try_into()
            .unwrap();
    }
    0
}
pub fn update_machine(
    machine: ((isize, isize, isize), (isize, isize, isize)),
) -> ((isize, isize, isize), (isize, isize, isize)) {
    let mut machine_copy = machine.clone();
    machine_copy.0 .2 += 10000000000000;
    machine_copy.1 .2 += 10000000000000;
    machine_copy
}

// ax+by=c
pub fn intersection_int(
    line1: (isize, isize, isize),
    line2: (isize, isize, isize),
) -> Option<(isize, isize)> {
    let d = line1.0 * line2.1 - line1.1 * line2.0;
    let dx = line1.2 * line2.1 - line1.1 * line2.2;
    let dy = line1.0 * line2.2 - line1.2 * line2.0;
    if d != 0 {
        if dx % d == 0 && dy % d == 0 {
            return Some((dx / d, dy / d));
        }
    }
    None
}

fn parse(s: &String) -> Vec<((isize, isize, isize), (isize, isize, isize))> {
    let mut machines = vec![];
    let re = Regex::new(r"Button A: X\+(?<a1>\d+), Y\+(?<a2>\d+)\nButton B: X\+(?<b1>\d+), Y\+(?<b2>\d+)\nPrize: X=(?<c1>\d+), Y=(?<c2>\d+)").unwrap();
    for m in re.captures_iter(s) {
        let a1 = m["a1"].parse().unwrap();
        let b1 = m["b1"].parse().unwrap();
        let c1 = m["c1"].parse().unwrap();
        let a2 = m["a2"].parse().unwrap();
        let b2 = m["b2"].parse().unwrap();
        let c2 = m["c2"].parse().unwrap();
        machines.push(((a1, b1, c1), (a2, b2, c2)));
    }
    machines
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279"
            .to_string()
    }

    #[test]
    fn test_part1() {
        assert_eq!(480, part1(&get_input()));
    }
}
