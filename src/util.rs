use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_data<T, F>(filename: &str, test: F) -> Vec<T>
where
    F: Fn(String) -> T,
{
    let mut parsed_lines: Vec<T> = Vec::new();
    if let Ok(lines) = crate::util::read_lines(filename) {
        for line in lines.flatten() {
            parsed_lines.push(test(line));
        }
    }
    parsed_lines
}
