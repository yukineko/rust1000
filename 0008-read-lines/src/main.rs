use std::fs::read_to_string;
use std::env;
use regex::Regex;
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("Reading file: {}", filename);
    let lines = read_lines(&filename);
    let re = Regex::new(r"Alice").unwrap();
    let mut alice_line = 0;
    lines.iter().for_each(|line: &String| {
        let Some(_caps) = re.captures(line) else {
            return;
        };
        alice_line += 1;
    });
    println!("Found {} lines with 'Alice'", alice_line);
}
#[allow(dead_code)]
fn read_lines(path:&str) -> Vec<String> {
    read_to_string(path)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}