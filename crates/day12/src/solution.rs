use helper::matrix::{self, find_neighbours_indices};
use std::collections::HashSet;

pub fn part1(input: &[String]) -> usize {
    let region = parse(input);
    let mut to_explore = get_indices_to_explore((region.len(), region[0].len()));
    let mut groups = vec![];
    while !to_explore.is_empty() {
        let next = *to_explore.iter().next().unwrap();
        groups.push(group_crops(&region, next, &mut to_explore))
    }
    let mut total_cost = 0;
    for group in groups {
        total_cost += calculate_cost(group, &region)
    }
    total_cost
}

pub fn part2(input: &[String]) -> usize {
    let region = parse(input);
    let mut to_explore = get_indices_to_explore((region.len(), region[0].len()));
    let mut groups = vec![];
    while !to_explore.is_empty() {
        let next = *to_explore.iter().next().unwrap();
        groups.push(group_crops(&region, next, &mut to_explore))
    }
    let mut total_cost = 0;
    for group in groups {
        total_cost += calculate_cost_2(group, &region)
    }
    total_cost
}

pub fn group_crops(
    region: &Vec<Vec<char>>,
    position: (usize, usize),
    to_explore: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    if !to_explore.contains(&position) {
        return vec![];
    } else {
        to_explore.remove(&position);
    }
    let mut group = vec![position];

    let neighbours = find_neighbours_indices(&region, position);
    for neighbour in neighbours {
        if region[neighbour.0][neighbour.1] == region[position.0][position.1] {
            group.append(&mut group_crops(region, neighbour, to_explore));
        }
    }
    group
}

fn calculate_cost(group: Vec<(usize, usize)>, region: &Vec<Vec<char>>) -> usize {
    let mut perimeter = 0;
    let area = group.len();
    let group_crop_type = region[group[0].0][group[0].1];
    for elem in &group {
        let mut same_type_neighbours_count = 0;
        let neighbours = matrix::find_neighbours_indices(region, *elem);
        for neighbour in neighbours {
            if region[neighbour.0][neighbour.1] == group_crop_type {
                same_type_neighbours_count += 1
            }
        }
        perimeter += 4 - same_type_neighbours_count
    }
    // println!(
    //     "{}:{:?}\nArea:{}\nPerimeter:{}\ncost:{}",
    //     group_crop_type,
    //     group,
    //     area,
    //     perimeter,
    //     area * perimeter
    // );
    area * perimeter
}

fn calculate_cost_2(group: Vec<(usize, usize)>, region: &Vec<Vec<char>>) -> usize {
    let mut perimeter = 0;
    let area = group.len();
    for elem in &group {
        let bottom_right = is_bottom_right_corner(elem, region);
        if bottom_right {
            perimeter += 1;
        }
        let bottom_left = is_bottom_left_corner(elem, region);
        if bottom_left {
            perimeter += 1;
        }
        let top_left = is_top_left_corner(elem, region);
        if top_left {
            perimeter += 1;
        }
        let top_right = is_top_right_corner(elem, region);
        if top_right {
            perimeter += 1;
        }
        let bottom_left_inner = is_bottom_left_inner_corner(elem, &group, region);
        if bottom_left_inner {
            perimeter += 1;
        }
        let bottom_right_inner = is_bottom_right_inner_corner(elem, &group, region);
        if bottom_right_inner {
            perimeter += 1;
        }
        let top_right_inner = is_top_right_inner_corner(elem, &group, region);
        if top_right_inner {
            perimeter += 1;
        }
        let top_left_inner = is_top_left_inner_corner(elem, &group, region);
        if top_left_inner {
            perimeter += 1;
        }
    }
    area * perimeter
}

fn is_bottom_right_corner(elem: &(usize, usize), region: &Vec<Vec<char>>) -> bool {
    let crop_type = region[elem.0][elem.1];
    if elem.1 < region[0].len() - 1 {
        if region[elem.0][elem.1 + 1] == crop_type {
            return false;
        }
    }
    if elem.0 < region[0].len() - 1 {
        if region[elem.0 + 1][elem.1] == crop_type {
            return false;
        }
    }
    true
}

fn is_bottom_left_corner(elem: &(usize, usize), region: &Vec<Vec<char>>) -> bool {
    let crop_type = region[elem.0][elem.1];
    if elem.1 > 0 {
        if region[elem.0][elem.1 - 1] == crop_type {
            return false;
        }
    }
    if elem.0 < region.len() - 1 {
        if region[elem.0 + 1][elem.1] == crop_type {
            return false;
        }
    }
    true
}

fn is_top_left_corner(elem: &(usize, usize), region: &Vec<Vec<char>>) -> bool {
    let crop_type = region[elem.0][elem.1];
    if elem.1 > 0 {
        if region[elem.0][elem.1 - 1] == crop_type {
            return false;
        }
    }
    if elem.0 > 0 {
        if region[elem.0 - 1][elem.1] == crop_type {
            return false;
        }
    }
    true
}

fn is_top_right_corner(elem: &(usize, usize), region: &Vec<Vec<char>>) -> bool {
    let crop_type = region[elem.0][elem.1];

    if elem.1 < region[0].len() - 1 {
        if region[elem.0][elem.1 + 1] == crop_type {
            return false;
        }
    }
    if elem.0 > 0 {
        if region[elem.0 - 1][elem.1] == crop_type {
            return false;
        }
    }
    true
}

fn is_bottom_right_inner_corner(
    elem: &(usize, usize),
    group: &Vec<(usize, usize)>,
    region: &Vec<Vec<char>>,
) -> bool {
    let crop_type = region[elem.0][elem.1];
    // right same type
    if elem.1 < region[0].len() - 1 && region[elem.0][elem.1 + 1] == crop_type {
        // bottom same type
        if elem.0 < region.len() - 1 && region[elem.0 + 1][elem.1] == crop_type {
            // opposite corner is not in the same group
            if !group.contains(&(elem.0 + 1, elem.1 + 1)) {
                return true;
            }
        }
    }
    false
}

fn is_bottom_left_inner_corner(
    elem: &(usize, usize),
    group: &Vec<(usize, usize)>,
    region: &Vec<Vec<char>>,
) -> bool {
    let crop_type = region[elem.0][elem.1];
    // left same type
    if elem.1 > 0 && region[elem.0][elem.1 - 1] == crop_type {
        // bottom same type
        if elem.0 < region.len() - 1 && region[elem.0 + 1][elem.1] == crop_type {
            // opposite corner is not in the same group
            if !group.contains(&(elem.0 + 1, elem.1 - 1)) {
                return true;
            }
        }
    }
    false
}

fn is_top_right_inner_corner(
    elem: &(usize, usize),
    group: &Vec<(usize, usize)>,
    region: &Vec<Vec<char>>,
) -> bool {
    let crop_type = region[elem.0][elem.1];
    // right same type
    if elem.1 < region[0].len() - 1 && region[elem.0][elem.1 + 1] == crop_type {
        // top same type
        if elem.0 > 0 && region[elem.0 - 1][elem.1] == crop_type {
            // opposite corner is not in the same group
            if !group.contains(&(elem.0 - 1, elem.1 + 1)) {
                return true;
            }
        }
    }
    false
}

fn is_top_left_inner_corner(
    elem: &(usize, usize),
    group: &Vec<(usize, usize)>,
    region: &Vec<Vec<char>>,
) -> bool {
    let crop_type = region[elem.0][elem.1];
    // left same type
    if elem.1 > 0 && region[elem.0][elem.1 - 1] == crop_type {
        // top same type
        if elem.0 > 0 && region[elem.0 - 1][elem.1] == crop_type {
            // opposite corner is not in the same group
            if !group.contains(&(elem.0 - 1, elem.1 - 1)) {
                return true;
            }
        }
    }
    false
}

pub fn get_indices_to_explore(region_size: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut to_explore = HashSet::new();
    for i in 0..region_size.0 {
        for j in 0..region_size.1 {
            to_explore.insert((i, j));
        }
    }
    to_explore
}

fn parse(strings: &[String]) -> Vec<Vec<char>> {
    let mut region = vec![];
    for line in strings {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        region.push(row);
    }
    region
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_1() -> Vec<String> {
        "AAAA
BBCD
BBCC
EEEC"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_2() -> Vec<String> {
        "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_3() -> Vec<String> {
        "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_4() -> Vec<String> {
        "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_5() -> Vec<String> {
        "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1_1() {
        assert_eq!(140, part1(&get_input_1()));
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(772, part1(&get_input_2()));
    }
    #[test]
    fn test_part1_3() {
        assert_eq!(1930, part1(&get_input_3()));
    }

    #[test]
    fn test_part2_1() {
        assert_eq!(80, part2(&get_input_1()));
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(436, part2(&get_input_2()));
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(1206, part2(&get_input_3()));
    }
    #[test]
    fn test_part2_4() {
        assert_eq!(236, part2(&get_input_4()));
    }
    #[test]
    fn test_part2_5() {
        assert_eq!(368, part2(&get_input_5()));
    }
}
