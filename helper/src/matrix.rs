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
