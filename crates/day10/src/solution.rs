use std::{collections::HashSet, fmt};

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    parsed.explore_start(0)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    parsed.explore_start_2(0)
}

#[derive(Debug)]
struct Map {
    terrain: Vec<Vec<u8>>,
}
impl Map {
    fn explore_start(&self, start_elem: u8) -> usize {
        let starting_positions = self.find_values(start_elem);
        let mut total = 0;
        for (r, c) in starting_positions {
            total += self.explore((r, c), &mut HashSet::new())
        }
        total
    }

    fn explore(&self, pos: (usize, usize), endings: &mut HashSet<(usize, usize)>) -> usize {
        if self.terrain[pos.0][pos.1] == 9 {
            if endings.contains(&pos) {
                return 0;
            }
            endings.insert(pos);
            return 1;
        }
        let nearby_pos = self.nearby_valid_positions(pos);
        let mut total = 0;
        for elem in nearby_pos {
            total += self.explore(elem, endings);
        }
        total
    }

    fn explore_start_2(&self, start_elem: u8) -> usize {
        let starting_positions = self.find_values(start_elem);
        let mut total = 0;
        for (r, c) in starting_positions {
            total += self.explore_2((r, c), &mut HashSet::new())
        }
        total
    }

    fn explore_2(&self, pos: (usize, usize), endings: &mut HashSet<(usize, usize)>) -> usize {
        if self.terrain[pos.0][pos.1] == 9 {
            return 1;
        }
        let nearby_pos = self.nearby_valid_positions(pos);
        let mut total = 0;
        for elem in nearby_pos {
            total += self.explore_2(elem, endings);
        }
        total
    }

    fn nearby_valid_positions(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut positions = vec![];
        let current_val = self.terrain[pos.0][pos.1];
        // up
        if pos.0 >= 1 {
            if self.terrain[pos.0 - 1][pos.1] == current_val + 1 {
                positions.push((pos.0 - 1, pos.1));
            }
        }
        // down
        if pos.0 + 1 < self.terrain.len() {
            if self.terrain[pos.0 + 1][pos.1] == current_val + 1 {
                positions.push((pos.0 + 1, pos.1));
            }
        }
        // left
        if pos.1 >= 1 {
            if self.terrain[pos.0][pos.1 - 1] == current_val + 1 {
                positions.push((pos.0, pos.1 - 1));
            }
        }
        // right
        if pos.1 + 1 < self.terrain.len() {
            if self.terrain[pos.0][pos.1 + 1] == current_val + 1 {
                positions.push((pos.0, pos.1 + 1));
            }
        }
        positions
    }

    fn find_values(&self, value: u8) -> Vec<(usize, usize)> {
        let mut found = vec![];
        for (r, row) in self.terrain.iter().enumerate() {
            for (c, elem) in row.iter().enumerate() {
                if *elem == value {
                    found.push((r, c));
                }
            }
        }
        found
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut print_string: String = "".to_string();
        for line in &self.terrain {
            for elem in line {
                if *elem == 11 {
                    print_string.push('.');
                } else {
                    print_string.push_str(&elem.to_string());
                }
            }
            print_string.push('\n');
        }
        write!(f, "{}", print_string)
    }
}

fn parse(strings: &[String]) -> Map {
    let mut v = vec![];
    for line in strings {
        let mut row = vec![];
        for c in line.chars() {
            if c != '.' {
                row.push(c.to_digit(10).unwrap() as u8);
            } else {
                row.push(11);
            }
        }
        v.push(row);
    }
    Map { terrain: v }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_part1_0() -> Vec<String> {
        "0123
1234
8765
9876"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    fn get_input_part1_1() -> Vec<String> {
        "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_part1_2() -> Vec<String> {
        "..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_part1_3() -> Vec<String> {
        "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input_part2_0() -> Vec<String> {
        ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9...."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    fn get_input() -> Vec<String> {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1_small_0() {
        assert_eq!(1, part1(&get_input_part1_0()));
    }
    #[test]
    fn test_part1_small_1() {
        assert_eq!(2, part1(&get_input_part1_1()));
    }
    #[test]
    fn test_part1_small_2() {
        assert_eq!(4, part1(&get_input_part1_2()));
    }
    #[test]
    fn test_part1_small_3() {
        assert_eq!(3, part1(&get_input_part1_3()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(36, part1(&get_input()));
    }

    #[test]
    fn test_part2_0() {
        assert_eq!(3, part2(&get_input_part2_0()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(81, part2(&get_input()));
    }
}
