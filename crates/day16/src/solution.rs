use pathfinding::prelude::astar_bag;
use pathfinding::prelude::dijkstra;
use std::collections::HashSet;
use std::{hash::Hash, usize};

pub fn part1(input: &[String]) -> usize {
    let (maze, start, dir, end) = parse(input);
    //let res = explore_maze(&maze, start, dir, end, &mut HashMap::new(), 0);
    let res_dijkstra = dijkstra(
        &(start, dir),
        |(pos, dir)| generate_moves_2(&maze, *pos, *dir),
        |(pos, _)| *pos == end,
    );
    res_dijkstra.unwrap().1
}

pub fn part2(input: &[String]) -> usize {
    let (maze, start, dir, end) = parse(input);
    //let res = explore_maze(&maze, start, dir, end, &mut HashMap::new(), 0);
    let all_paths = astar_bag(
        &(start, dir),
        |(pos, dir)| generate_moves_2(&maze, *pos, *dir),
        |_| 0,
        |(pos, _)| *pos == end,
    )
    .unwrap();
    let mut visited_nodes = HashSet::new();
    for elem in all_paths.0 {
        for node in elem {
            visited_nodes.insert(node.0);
        }
    }
    visited_nodes.len()
}

fn _generate_moves(
    maze: &Vec<Vec<bool>>,
    pos: (usize, usize),
    dir: Direction,
) -> Vec<(((usize, usize), Direction), usize)> {
    let mut moves = vec![];
    let next_pos = dir.next_pos(pos);
    if maze[next_pos.0][next_pos.1] {
        moves.push(((next_pos, dir), 1));
    }
    moves.push(((pos, dir.rotate(true)), 1000));
    moves.push(((pos, dir.rotate(false)), 1000));
    moves
}

fn generate_moves_2(
    maze: &Vec<Vec<bool>>,
    pos: (usize, usize),
    dir: Direction,
) -> Vec<(((usize, usize), Direction), usize)> {
    let mut moves = vec![];
    let next_pos = dir.next_pos(pos);
    if maze[next_pos.0][next_pos.1] {
        moves.push(((next_pos, dir), 1));
    }
    let pos_clock = dir.rotate(true).next_pos(pos);
    if maze[pos_clock.0][pos_clock.1] {
        moves.push(((pos, dir.rotate(true)), 1000));
    }
    let pos_anticlock = dir.rotate(false).next_pos(pos);
    if maze[pos_anticlock.0][pos_anticlock.1] {
        moves.push(((pos, dir.rotate(false)), 1000));
    }
    moves
}

fn parse(strings: &[String]) -> (Vec<Vec<bool>>, (usize, usize), Direction, (usize, usize)) {
    let mut structure = vec![];
    let mut pos = (0, 0);
    let mut end = (0, 0);
    for (i, line) in strings.iter().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            match c {
                'E' => {
                    row.push(true);
                    end = (i, j)
                }
                'S' => {
                    row.push(true);
                    pos = (i, j)
                }
                '.' => row.push(true),
                '#' => row.push(false),
                _ => unreachable!("Unexpected char in input"),
            }
        }
        structure.push(row);
    }
    (structure, pos, Direction::Right, end)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn next_pos(self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }
    fn rotate(self, clockise: bool) -> Direction {
        match self {
            Direction::Up => {
                if clockise {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Down => {
                if clockise {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Left => {
                if clockise {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
            Direction::Right => {
                if clockise {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    fn get_input_2() -> Vec<String> {
        "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(7036, part1(&get_input()));
    }
    #[test]
    fn test_part1_2() {
        assert_eq!(11048, part1(&get_input_2()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(45, part2(&get_input()));
    }
    #[test]
    fn test_part2_2() {
        assert_eq!(64, part2(&get_input_2()));
    }
}
