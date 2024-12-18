use helper::matrix;
use pathfinding::prelude::dijkstra;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    find_shortest_path(&parsed, 71, 1024).unwrap().1
}

pub fn part2(input: &[String]) -> (usize, usize) {
    let parsed = parse(input);
    find_path_until_fail(&parsed, 71, 1025)
}

fn find_path_until_fail(
    corrupted_bytes: &Vec<(usize, usize)>,
    size: usize,
    start_index: usize,
) -> (usize, usize) {
    for i in start_index..corrupted_bytes.len() {
        let res = find_shortest_path(&corrupted_bytes, size, i);
        if !res.is_some() {
            return corrupted_bytes[i - 1];
        }
    }
    (0, 0)
}

fn find_shortest_path(
    corrupted_bytes: &Vec<(usize, usize)>,
    size: usize,
    n_bytes: usize,
) -> Option<(Vec<(usize, usize)>, usize)> {
    let considered_bytes = corrupted_bytes[..n_bytes].to_vec();
    let mut maze = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push('.');
        }
        maze.push(row);
    }
    for (i, j) in &considered_bytes {
        maze[*i][*j] = '#'
    }
    let result = dijkstra(
        &(0, 0),
        |&pos| get_moves(&maze, pos).into_iter().map(|p| (p, 1)),
        |&p| p == (size - 1, size - 1),
    );

    result
}

fn get_moves(maze: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    let possible_moves = matrix::find_neighbours_indices(maze, pos);
    possible_moves
        .iter()
        .filter(|(i, j)| maze[*i][*j] == '.')
        .map(|x| *x)
        .collect()
}

fn parse(strings: &[String]) -> Vec<(usize, usize)> {
    let mut corrupted_bytes = vec![];
    for line in strings {
        let mut values = line.split(',');
        let col_num: usize = values.next().unwrap().parse().unwrap();
        let row_num: usize = values.next().unwrap().parse().unwrap();
        corrupted_bytes.push((row_num, col_num));
    }
    corrupted_bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .lines()
        .map(|s| String::from(s.trim()))
        .collect()
    }

    #[test]
    fn test_part1() {
        let parsed = parse(&get_input());
        let res = find_shortest_path(&parsed, 7, 12).unwrap().1;
        assert_eq!(22, res);
    }

    #[test]
    fn test_part2() {
        let parsed = parse(&get_input());
        let res = find_path_until_fail(&parsed, 7, 13);
        assert_eq!((6, 1), res);
    }
}
