use helper;

pub fn part1(input: &[String]) -> usize {
    let parsed = parse(input);
    find_xmas(parsed)
}

pub fn part2(input: &[String]) -> usize {
    let parsed = parse(input);
    find_cross_mas(parsed)
}

fn find_cross_mas(matrix: Vec<Vec<char>>) -> usize {
    let mut total = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] == 'A' {
                total += verify_cross_mas((i, j), &matrix)
            }
        }
    }
    total
}

fn find_xmas(matrix: Vec<Vec<char>>) -> usize {
    let mut total = 0;
    // horizontal and reverse horizontal
    for row in &matrix {
        for word in row.windows(4) {
            let string_word: String = word.iter().cloned().collect();
            if string_word.eq("XMAS") {
                total += 1
            } else if string_word.eq("SAMX") {
                total += 1
            }
        }
    }
    let matrix_transposed = helper::matrix::transpose(&matrix);
    // vertical and reverse vertical
    for row in &matrix_transposed {
        for word in row.windows(4) {
            let string_word: String = word.iter().cloned().collect();
            if string_word.eq("XMAS") {
                total += 1
            } else if string_word.eq("SAMX") {
                total += 1
            }
        }
    }
    // diagonal and reverse diagonal
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'X' {
                total += find_xmas_diagonal((i, j), (1, -1), &matrix);
                total += find_xmas_diagonal((i, j), (1, 1), &matrix);
                total += find_xmas_diagonal((i, j), (-1, -1), &matrix);
                total += find_xmas_diagonal((i, j), (-1, 1), &matrix);
            }
        }
    }
    total
}

fn find_xmas_diagonal(pos: (usize, usize), dir: (i8, i8), matrix: &Vec<Vec<char>>) -> usize {
    let current_char = matrix[pos.0][pos.1];
    let mut next_pos = pos;
    if dir.0 == -1 {
        if pos.0 == 0 {
            return 0;
        } else {
            next_pos.0 = pos.0 - 1;
        }
    }
    if dir.1 == -1 {
        if pos.1 == 0 {
            return 0;
        } else {
            next_pos.1 = pos.1 - 1
        }
    }
    if dir.0 == 1 {
        if pos.0 == matrix.len() - 1 {
            return 0;
        } else {
            next_pos.0 = pos.0 + 1
        }
    }
    if dir.1 == 1 {
        if pos.1 == matrix[0].len() - 1 {
            return 0;
        } else {
            next_pos.1 = pos.1 + 1
        }
    }
    let next_char = matrix[next_pos.0][next_pos.1];
    match (current_char, next_char) {
        ('X', 'M') | ('M', 'A') => find_xmas_diagonal(next_pos, dir, matrix),
        ('A', 'S') => 1,
        _ => 0,
    }
}

fn verify_cross_mas(pos: (usize, usize), matrix: &Vec<Vec<char>>) -> usize {
    let pos_top_left = (pos.0 - 1, pos.1 - 1);
    let pos_bottom_right = (pos.0 + 1, pos.1 + 1);
    let pos_top_right = (pos.0 - 1, pos.1 + 1);
    let pos_bottom_left = (pos.0 + 1, pos.1 - 1);

    let mut valid = 0;
    if (matrix[pos_top_left.0][pos_top_left.1] == 'M'
        && matrix[pos_bottom_right.0][pos_bottom_right.1] == 'S')
        || (matrix[pos_top_left.0][pos_top_left.1] == 'S'
            && matrix[pos_bottom_right.0][pos_bottom_right.1] == 'M')
    {
        valid += 1
    }
    if (matrix[pos_top_right.0][pos_top_right.1] == 'M'
        && matrix[pos_bottom_left.0][pos_bottom_left.1] == 'S')
        || (matrix[pos_top_right.0][pos_top_right.1] == 'S'
            && matrix[pos_bottom_left.0][pos_bottom_left.1] == 'M')
    {
        valid += 1
    }
    if valid == 2 {
        return 1;
    }
    0
}

fn parse(strings: &[String]) -> Vec<Vec<char>> {
    let mut matrix = vec![];
    for line in strings {
        matrix.push(line.chars().collect())
    }
    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_small() -> Vec<String> {
        "..X...
.SAMX.
.A..A.
XMAS.S
.X...."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    fn get_input() -> Vec<String> {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    fn get_input_small2() -> Vec<String> {
        "M.S
.A.
M.S"
        .lines()
        .map(|s| String::from(s.trim()))
        .collect()
    }

    fn get_input2() -> Vec<String> {
        ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
.........."
            .lines()
            .map(|s| String::from(s.trim()))
            .collect()
    }

    #[test]
    fn test_part1_small() {
        assert_eq!(4, part1(&get_input_small()));
    }

    #[test]
    fn test_part1() {
        assert_eq!(18, part1(&get_input()));
    }

    #[test]
    fn test_part12_small() {
        assert_eq!(1, part2(&get_input_small2()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2(&get_input2()));
    }
}
