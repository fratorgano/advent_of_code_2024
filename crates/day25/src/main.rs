use std::time::Instant;

use helper::input;

mod solution;

fn main() {
    // day25
    let input = input::read_input_lines_vec("day25");

    let before1 = Instant::now();
    let res1 = solution::part1(&input);
    println!("Part 1 result: {} in {:?}", res1, before1.elapsed());
}
