use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};

// Takes a csv row of integers and returns a vector with corresponding values
// Chars may or may not be prefixed with sign (+ or -).
// Whitespaces are ignored
// Empty values are not allows exept at end. And is then not counted
// Empty string -> empty vector
//

#[allow(dead_code)]
pub fn parse_string_of_insts_to_vec(s: &str) -> Vec<i32> {
    let s = s.trim();

    if s.is_empty() {
        return Vec::new();
    }

    s.split(',')
        .map(|x| x.trim().parse().expect("Not an integer"))
        .collect()
}

// Private helper to open file. In the context of AoC its ok to panic
fn open_filename_to_file(input_filename: &str) -> std::fs::File {
    match fs::File::open(input_filename) {
        Err(why) => panic!(
            "Failed to open {}: {}",
            input_filename,
            Error::description(&why)
        ),
        Ok(file) => file,
    }
}

// Returns a vector of ints, taken from an aoc2018 provided input file (one integer per row)
pub fn aoc2018_integer_file_to_vector(input_filename: &str) -> Vec<i32> {
    let file = open_filename_to_file(input_filename);
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.unwrap().trim().parse().expect("Not an integer"))
        .collect()
}

// Returns a vector of trimmed strings with row content, taken from an aoc2018 provided input file
pub fn aoc2018_row_file_to_vector(input_filename: &str) -> Vec<String> {
    let file = open_filename_to_file(input_filename);
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
