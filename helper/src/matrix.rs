pub fn print_matrix<T: std::fmt::Display>(matrix: &Vec<Vec<T>>) {
    for line in matrix {
        for elem in line {
            print!(" {} ", elem)
        }
        println!();
    }
    println!();
}

pub fn print_matrix_debug<T: std::fmt::Debug>(matrix: &Vec<Vec<T>>) {
    for line in matrix {
        for elem in line {
            print!(" {:?} ", elem)
        }
        println!();
    }
    println!();
}

pub fn transpose<T: std::clone::Clone>(original: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!original.is_empty());
    let len = original[0].len();
    let mut iters: Vec<_> = original.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap().clone())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn find_neighbours_indices<T>(
    matrix: &Vec<Vec<T>>,
    pos: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    if pos.1 > 0 {
        neighbours.push((pos.0, pos.1 - 1));
    }
    if pos.1 < matrix[0].len() - 1 {
        neighbours.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        neighbours.push((pos.0 - 1, pos.1));
    }
    if pos.0 < matrix.len() - 1 {
        neighbours.push((pos.0 + 1, pos.1));
    }

    neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_neighbours() {
        let matrix = vec![vec![1, 2], vec![3, 4]];
        assert_eq!(
            vec![(0, 1), (1, 0)],
            find_neighbours_indices(&matrix, (0, 0))
        );
        assert_eq!(
            vec![(0, 0), (1, 1)],
            find_neighbours_indices(&matrix, (0, 1))
        );
        assert_eq!(
            vec![(1, 1), (0, 0)],
            find_neighbours_indices(&matrix, (1, 0))
        );
        assert_eq!(
            vec![(1, 0), (0, 1)],
            find_neighbours_indices(&matrix, (1, 1))
        );
    }
}
