use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn read_lines(filename: &str) -> io::Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        lines.push(line?);
    }
    return Ok(lines);
}