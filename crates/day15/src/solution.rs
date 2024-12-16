use core::fmt;
use std::collections::HashSet;

pub fn part1(input: &[String]) -> usize {
    let mut warehouse = parse(input);
    warehouse.run_robot();
    warehouse.score()
}

pub fn part2(input: &[String]) -> usize {
    let mut warehouse = parse2(input);
    warehouse.run_robot_2();
    warehouse.score()
}

struct Warehouse {
    map: Vec<Vec<Option<Obstacle>>>,
    robot_pos: (usize, usize),
    robot_commands: Vec<Direction>,
}
impl Warehouse {
    fn run_robot(&mut self) {
        for direction in &self.robot_commands {
            if let Some(free_space) = self.try_to_move(self.robot_pos, *direction) {
                self.map[free_space.0][free_space.1] = Some(Obstacle::Box);
                let new_robot_pos = match direction {
                    Direction::Up => (self.robot_pos.0 - 1, self.robot_pos.1),
                    Direction::Down => (self.robot_pos.0 + 1, self.robot_pos.1),
                    Direction::Left => (self.robot_pos.0, self.robot_pos.1 - 1),
                    Direction::Right => (self.robot_pos.0, self.robot_pos.1 + 1),
                };
                self.map[new_robot_pos.0][new_robot_pos.1] = None;
                self.robot_pos = new_robot_pos;
            }
        }
    }
    fn try_to_move(&self, pos: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
        let dir_coord = match direction {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        };
        let elem = self.map[dir_coord.0][dir_coord.1];
        match elem {
            Some(Obstacle::Box) => self.try_to_move(dir_coord, direction),
            Some(Obstacle::Wall) => None,
            None => Some(dir_coord),
            _ => unreachable!(),
        }
    }
    fn run_robot_2(&mut self) {
        for direction in &self.robot_commands {
            // println!("{:?}", direction);
            if let Some(free_space) = self.try_to_move_2(self.robot_pos, *direction) {
                // println!("{:?}", free_space);
                let new_robot_pos = match direction {
                    Direction::Up => (self.robot_pos.0 - 1, self.robot_pos.1),
                    Direction::Down => (self.robot_pos.0 + 1, self.robot_pos.1),
                    Direction::Left => (self.robot_pos.0, self.robot_pos.1 - 1),
                    Direction::Right => (self.robot_pos.0, self.robot_pos.1 + 1),
                };
                if *direction == Direction::Left || *direction == Direction::Right {
                    self.map[free_space.iter().next().unwrap().0]
                        .remove(free_space.iter().next().unwrap().1);
                    self.map[self.robot_pos.0].insert(self.robot_pos.1, None);
                } else {
                    let mut to_move = free_space.into_iter().collect::<Vec<_>>();
                    to_move.sort();
                    if *direction == Direction::Up {
                        to_move.sort();
                        for (i, j) in to_move {
                            self.map[i][j] = self.map[i + 1][j];
                            self.map[i + 1][j] = None;
                        }
                    } else {
                        to_move.sort_by(|a, b| b.cmp(a));
                        for (i, j) in to_move {
                            self.map[i][j] = self.map[i - 1][j];
                            self.map[i - 1][j] = None;
                        }
                    }
                }
                self.robot_pos = new_robot_pos;
            }
        }
    }
    fn try_to_move_2(
        &self,
        pos: (usize, usize),
        direction: Direction,
    ) -> Option<HashSet<(usize, usize)>> {
        let dir_coord = match direction {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        };
        let elem = self.map[dir_coord.0][dir_coord.1];
        match elem {
            Some(Obstacle::BoxStart) => {
                if direction == Direction::Down || direction == Direction::Up {
                    let left_opt = self.try_to_move_2(dir_coord, direction);
                    let right_opt = self.try_to_move_2((dir_coord.0, dir_coord.1 + 1), direction);
                    if left_opt.is_some() && right_opt.is_some() {
                        let mut things_to_move = HashSet::new();
                        let left = left_opt.unwrap();
                        let right = right_opt.unwrap();
                        for elem in left {
                            things_to_move.insert(elem);
                        }
                        for elem in right {
                            things_to_move.insert(elem);
                        }
                        things_to_move.insert(dir_coord);
                        Some(things_to_move)
                    } else {
                        None
                    }
                } else {
                    self.try_to_move_2(dir_coord, direction)
                }
            }
            Some(Obstacle::BoxEnd) => {
                if direction == Direction::Down || direction == Direction::Up {
                    let left_opt = self.try_to_move_2((dir_coord.0, dir_coord.1 - 1), direction);
                    let right_opt = self.try_to_move_2(dir_coord, direction);
                    if left_opt.is_some() && right_opt.is_some() {
                        let mut things_to_move = HashSet::new();
                        let left = left_opt.unwrap();
                        let right = right_opt.unwrap();
                        for elem in left {
                            things_to_move.insert(elem);
                        }
                        for elem in right {
                            things_to_move.insert(elem);
                        }
                        things_to_move.insert(dir_coord);
                        Some(things_to_move)
                    } else {
                        None
                    }
                } else {
                    self.try_to_move_2(dir_coord, direction)
                }
            }
            Some(Obstacle::Wall) => None,
            None => {
                let mut set = HashSet::new();
                set.insert(dir_coord);
                Some(set)
            }
            _ => unreachable!(),
        }
    }

    fn score(&self) -> usize {
        let mut total = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if let Some(Obstacle::Box) = elem {
                    total += i * 100 + j
                } else if let Some(Obstacle::BoxStart) = elem {
                    total += i * 100 + j
                }
            }
        }
        total
    }
}
impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_string();
        for (i, row) in self.map.iter().enumerate() {
            for (j, elem) in row.iter().enumerate() {
                if (i, j) == self.robot_pos {
                    out.push('@');
                    continue;
                }
                match elem {
                    None => out.push('.'),
                    Some(x) => out.push_str(&format!("{}", x)),
                }
            }
            out.push('\n')
        }
        write!(f, "{}", out)
    }
}

fn parse(strings: &[String]) -> Warehouse {
    let mut movements = vec![];
    let mut map = vec![];
    let mut starting_pos = (0, 0);

    let mut first_part = true;
    for (i, line) in strings.iter().enumerate() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                let obstacle = match c {
                    '#' => Some(Obstacle::Wall),
                    'O' => Some(Obstacle::Box),
                    '@' => {
                        starting_pos = (i, j);
                        None
                    }
                    '.' => None,
                    _ => unreachable!("Unexpected obstacle in the input"),
                };
                row.push(obstacle);
            }
            map.push(row);
        } else {
            for c in line.chars() {
                let dir = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!("Unexpected direction in the input"),
                };
                movements.push(dir);
            }
        }
    }
    Warehouse {
        map,
        robot_pos: starting_pos,
        robot_commands: movements,
    }
}

fn parse2(strings: &[String]) -> Warehouse {
    let mut movements = vec![];
    let mut map = vec![];
    let mut starting_pos = (0, 0);

    let mut first_part = true;
    for (i, line) in strings.iter().enumerate() {
        if line == "" {
            first_part = false;
            continue;
        }
        if first_part {
            let mut row = vec![];
            for (j, c) in line.chars().enumerate() {
                let obstacle = match c {
                    '#' => Some(Obstacle::Wall),
                    'O' => Some(Obstacle::Box),
                    '@' => {
                        starting_pos = (i, 2 * j);
                        None
                    }
                    '.' => None,
                    _ => unreachable!("Unexpected obstacle in the input"),
                };
                if let Some(Obstacle::Box) = obstacle {
                    row.push(Some(Obstacle::BoxStart));
                    row.push(Some(Obstacle::BoxEnd));
                } else {
                    row.push(obstacle);
                    row.push(obstacle);
                }
            }
            map.push(row);
        } else {
            for c in line.chars() {
                let dir = match c {
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    _ => unreachable!("Unexpected direction in the input"),
                };
                movements.push(dir);
            }
        }
    }
    Warehouse {
        map,
        robot_pos: starting_pos,
        robot_commands: movements,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Obstacle {
    Box,
    Wall,
    BoxStart,
    BoxEnd,
}
impl fmt::Display for Obstacle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Obstacle::Box => write!(f, "O"),
            Obstacle::Wall => write!(f, "#"),
            Obstacle::BoxStart => write!(f, "["),
            Obstacle::BoxEnd => write!(f, "]"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_small() -> Vec<String> {
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }
    fn get_input() -> Vec<String> {
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1_small() {
        assert_eq!(2028, part1(&get_input_small()));
    }
    #[test]
    fn test_part1() {
        assert_eq!(10092, part1(&get_input()));
    }

    #[test]
    fn test_part2_long() {
        assert_eq!(9021, part2(&get_input()));
    }
}
