use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn iterate_file_lines(file_path: &str) -> impl Iterator<Item = String> {
    let file = File::open(Path::new("./input").join(file_path)).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    buf_reader.lines().map(|line| line.expect("Error reading next line"))
}
