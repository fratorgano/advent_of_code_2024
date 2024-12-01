use std::env;
use std::fs;
use std::io::BufRead;

pub fn read_input_lines_vec(day: &str) -> Vec<String> {
    let mut dir = env::current_dir().unwrap();
    dir.push("inputs");
    dir.push(format!("{}.txt", day));
    let file = fs::File::open(dir).expect(&format!("File {day}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}
