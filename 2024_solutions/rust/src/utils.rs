use std::fs;
use std::io::Read;

pub fn read_file_strings(string: &str) -> Vec<String> {
    let mut data = String::new();
    let mut file = fs::File::open(string).unwrap();
    file.read_to_string(&mut data).unwrap();
    let file_lines: Vec<String> = data.split("\n")
        .map(String::from)
        .collect();

    return file_lines;
}
