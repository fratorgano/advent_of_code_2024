use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    find_shortcuts(parsed.0, parsed.1)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    find_shortcuts_2(parsed.0)
}

fn find_shortcuts(path: Vec<(usize, usize)>, map: Vec<Vec<char>>) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for point_index in 0..path.len() {
        let (_before, after) = path.split_at(point_index);
        for point2_index in 0..after.len() {
            let dis = distance(path[point_index], path[point_index + point2_index]);
            let med_point = (
                (path[point_index].0 + path[point_index + point2_index].0) / 2,
                (path[point_index].1 + path[point_index + point2_index].1) / 2,
            );
            if dis == 2 && map[med_point.0][med_point.1] == '#' {
                // println!(
                //     "{:?}->({:?})->{:?} ",
                //     path[point_index],
                //     med_point,
                //     path[point_index + point2_index]
                // );
                // println!("Index:{:?}->{:?}", point_index, point_index + point2_index);
                // println!(
                //     "Savings:{:?}-{:?} = {}\n",
                //     point_index + point2_index,
                //     point_index,
                //     point2_index - dis
                // );
                if point2_index - dis > 0 {
                    counts
                        .entry(point2_index - dis)
                        .and_modify(|x| {
                            *x = *x + 1;
                        })
                        .or_insert(1);
                }
            }
        }
    }
    counts
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| *v)
        .sum()
}

fn find_shortcuts_2(path: Vec<(usize, usize)>) -> usize {
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for point_index in 0..path.len() {
        let (_before, after) = path.split_at(point_index);
        for point2_index in 0..after.len() {
            let dis = distance(path[point_index], path[point_index + point2_index]);
            if dis <= 20 {
                if point2_index - dis > 0 {
                    counts
                        .entry(point2_index - dis)
                        .and_modify(|x| {
                            *x = *x + 1;
                        })
                        .or_insert(1);
                }
            }
        }
    }
    counts
        .iter()
        .filter(|(k, _)| **k >= 100)
        .map(|(_, v)| *v)
        .sum()
}

fn distance(point1: (usize, usize), point2: (usize, usize)) -> usize {
    point1.0.abs_diff(point2.0) + point1.1.abs_diff(point2.1)
}

fn parse(strings: &[String]) -> (Vec<(usize, usize)>, Vec<Vec<char>>) {
    let mut map = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in strings.iter().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (i, j);
                row.push('.');
            } else if c == 'E' {
                end = (i, j);
                row.push('.');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    let mut start_end_path = vec![];
    let mut visited = HashSet::new();
    let mut pos = start;
    while pos != end {
        visited.insert(pos);
        start_end_path.push(pos);
        // check up
        if map[pos.0 - 1][pos.1] == '.' && !visited.contains(&(pos.0 - 1, pos.1)) {
            pos = (pos.0 - 1, pos.1);
            continue;
        }
        // check down
        if map[pos.0 + 1][pos.1] == '.' && !visited.contains(&(pos.0 + 1, pos.1)) {
            pos = (pos.0 + 1, pos.1);
            continue;
        }
        // check left
        if map[pos.0][pos.1 - 1] == '.' && !visited.contains(&(pos.0, pos.1 - 1)) {
            pos = (pos.0, pos.1 - 1);
            continue;
        }
        // check right
        if map[pos.0][pos.1 + 1] == '.' && !visited.contains(&(pos.0, pos.1 + 1)) {
            pos = (pos.0, pos.1 + 1);
            continue;
        }
        unreachable!("Something went wrong with finding the path between start and end");
    }
    start_end_path.push(end);
    (start_end_path, map)
}

#[cfg(test)]
mod tests {}
