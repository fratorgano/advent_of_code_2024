use std::{collections::HashSet, fmt};

pub fn part1(input: &[String]) -> usize {
    let mut parsed = parse(input);

    let mut visited_positions = HashSet::new();
    visited_positions.insert(parsed.guard);

    while let Some(pos) = parsed.move_guard() {
        visited_positions.insert(pos);
    }

    visited_positions.len()
}

pub fn part2(input: &[String]) -> usize {
    let mut parsed = parse(input);

    let starting_position = parsed.guard;

    // do a first run to see where we can place obstacles
    let mut visited_positions = HashSet::new();
    visited_positions.insert(parsed.guard);

    visited_positions.remove(&starting_position);

    while let Some(pos) = parsed.move_guard() {
        visited_positions.insert(pos);
    }

    let mut loop_count = 0;
    for position in visited_positions {
        // reset guard
        parsed.guard = starting_position;
        parsed.direction = Direction::North;
        let mut visited_pos_dir = HashSet::new();
        parsed.walls.push(position);
        while let Some(pos) = parsed.move_guard() {
            if !visited_pos_dir.insert((pos, parsed.direction)) {
                loop_count += 1;
                break;
            }
        }
        parsed.walls.remove(parsed.walls.len() - 1);
    }

    loop_count
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    r: usize,
    c: usize,
}

#[derive(Debug)]
struct Room {
    walls: Vec<Pos>,
    guard: Pos,
    direction: Direction,
    size: (usize, usize),
}

impl Room {
    fn move_guard(&mut self) -> Option<Pos> {
        let possible_pos_opt = self.next_pos();
        if let Some(possible_pos) = possible_pos_opt {
            if self.walls.contains(&possible_pos) {
                self.direction = self.direction.rotate90r();
                return Some(self.guard);
            } else {
                self.guard = possible_pos;
                return Some(self.guard);
            }
        }
        None
    }

    fn next_pos(&self) -> Option<Pos> {
        match self.direction {
            Direction::Est => {
                let next_c = self.guard.c + 1;
                if next_c < self.size.1 {
                    return Some(Pos {
                        r: self.guard.r,
                        c: next_c,
                    });
                }
            }
            Direction::Sud => {
                let next_r = self.guard.r + 1;
                if next_r < self.size.0 {
                    return Some(Pos {
                        r: next_r,
                        c: self.guard.c,
                    });
                }
            }
            Direction::Ovest => {
                let next_c = self.guard.c.checked_sub(1);
                if let Some(c) = next_c {
                    return Some(Pos { r: self.guard.r, c });
                }
            }
            Direction::North => {
                let next_r = self.guard.r.checked_sub(1);
                if let Some(r) = next_r {
                    return Some(Pos { r, c: self.guard.c });
                }
            }
        }
        None
    }
}

impl fmt::Display for Room {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        let mut print_string: String = "".to_string();
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                let pos = Pos { r, c };
                if self.walls.contains(&pos) {
                    print_string.push_str(" # ");
                } else if self.guard == pos {
                    match self.direction {
                        Direction::Est => print_string.push_str(" > "),
                        Direction::Sud => print_string.push_str(" v "),
                        Direction::Ovest => print_string.push_str(" < "),
                        Direction::North => print_string.push_str(" ^ "),
                    }
                } else {
                    print_string.push_str(" . ");
                }
            }
            print_string.push('\n');
        }
        write!(f, "{}", print_string)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Est,
    Sud,
    Ovest,
    North,
}
impl Direction {
    fn rotate90r(&self) -> Direction {
        match self {
            Direction::Est => Direction::Sud,
            Direction::Sud => Direction::Ovest,
            Direction::Ovest => Direction::North,
            Direction::North => Direction::Est,
        }
    }
}

fn parse(strings: &[String]) -> Room {
    let mut walls = vec![];
    let mut guard: Pos = { Pos { r: 0, c: 0 } };
    for (i, line) in strings.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '#' => walls.push(Pos { r: i, c: j }),
                '^' => {
                    guard.r = i;
                    guard.c = j;
                }
                '.' => (),
                _ => unreachable!("Unexpected char in input!"),
            }
        }
    }
    Room {
        walls,
        guard,
        direction: Direction::North,
        size: (strings.len(), strings[0].len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(41, part1(&get_input()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2(&get_input()));
    }
}
