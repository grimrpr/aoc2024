use std::fs::File;
use std::io::BufReader;

pub fn print_answer() {
    let reader = BufReader::new(File::open("data/input_day01").unwrap());
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
}
