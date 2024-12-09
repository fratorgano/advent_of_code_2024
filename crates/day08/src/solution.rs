use std::collections::{HashMap, HashSet};

pub fn part1(input: &[String]) -> usize {
    let (map, limits) = parse(input);
    let mut antinodes_set = HashSet::new();
    for (_, antennas) in map {
        for antenna in antennas.iter() {
            for antenna2 in antennas.iter() {
                let antinode_opt = find_antinodes(antenna, antenna2, limits);
                if let Some(antinode) = antinode_opt {
                    antinodes_set.insert(antinode);
                }
            }
        }
    }
    antinodes_set.len()
}

pub fn part2(input: &[String]) -> usize {
    let (map, limits) = parse(input);
    let mut antinodes_set = HashSet::new();
    for (_, antennas) in map {
        for antenna in antennas.iter() {
            for antenna2 in antennas.iter() {
                let antinodes = find_antinodes_2(antenna, antenna2, limits);
                for antinode in antinodes {
                    antinodes_set.insert(antinode);
                }
            }
        }
    }
    antinodes_set.len()
}

fn find_antinodes(pos1: &Pos, pos2: &Pos, limits: (isize, isize)) -> Option<Pos> {
    let r_diff = pos1.r - pos2.r;
    let c_diff = pos1.c - pos2.c;

    if r_diff == 0 && c_diff == 0 {
        return None;
    }

    pos1.create_with_offset((r_diff, c_diff), limits)
}

fn find_antinodes_2(pos1: &Pos, pos2: &Pos, limits: (isize, isize)) -> Vec<Pos> {
    let r_diff = pos1.r - pos2.r;
    let c_diff = pos1.c - pos2.c;

    if r_diff == 0 && c_diff == 0 {
        return vec![];
    }

    pos1.create_multiple_with_offset((r_diff, c_diff), limits)
}

fn parse(strings: &[String]) -> (HashMap<char, Vec<Pos>>, (isize, isize)) {
    let mut map = HashMap::new();
    for (i, line) in strings.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                '.' | '\n' => {}
                c => {
                    map.entry(c)
                        .and_modify(|v: &mut Vec<Pos>| {
                            v.push(Pos {
                                r: i as isize,
                                c: j as isize,
                            })
                        })
                        .or_insert(vec![Pos {
                            r: i as isize,
                            c: j as isize,
                        }]);
                }
            }
        }
    }
    (map, (strings.len() as isize, strings[0].len() as isize))
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    r: isize,
    c: isize,
}
impl Pos {
    fn create_with_offset(&self, offset: (isize, isize), limits: (isize, isize)) -> Option<Pos> {
        let new_r = self.r + offset.0;
        let new_c = self.c + offset.1;
        if new_r >= 0 && new_r < limits.0 && new_c >= 0 && new_c < limits.1 {
            return Some(Pos { r: new_r, c: new_c });
        }
        None
    }
    fn create_multiple_with_offset(
        &self,
        offset: (isize, isize),
        limits: (isize, isize),
    ) -> Vec<Pos> {
        let mut positions = vec![];
        let mut i = 0;
        loop {
            let new_r = self.r + offset.0 * i;
            let new_c = self.c + offset.1 * i;
            if new_r >= 0 && new_r < limits.0 && new_c >= 0 && new_c < limits.1 {
                positions.push(Pos { r: new_r, c: new_c });
            } else {
                break;
            }
            i += 1
        }
        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(14, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(34, part2(&get_input()));
    }
}
