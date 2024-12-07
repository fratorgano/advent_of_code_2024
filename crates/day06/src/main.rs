use std::time::Instant;

use helper::input;

mod solution;

fn main() {
    // day06
    let input = input::read_input_lines_vec("day06");

    let before1 = Instant::now();
    let res1 = solution::part1(&input);
    println!("Part 1 result: {} in {:?}", res1, before1.elapsed());

    let before2 = Instant::now();
    let res2 = solution::part2(&input);
    println!("Part 2 result: {} in {:?}", res2, before2.elapsed());
}
