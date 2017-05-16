extern crate libdiff;
use libdiff::{build_lcs_table, print_diff};
use std::io::{Read, BufReader, BufRead};
use std::env;
use std::fs::File;


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        panic!("diff requires two paths to text files as arguments");
    }

    let file_a = File::open(&args[1]).unwrap();
    let file_b = File::open(&args[2]).unwrap();

    let lines_a = read_lines(BufReader::new(file_a));
    let lines_b = read_lines(BufReader::new(file_b));

    let table = build_lcs_table(&lines_a, &lines_b);
    print_diff(&table, &lines_a, &lines_b, lines_a.len(), lines_b.len());
}

/// Read from a reader to a Vec<String> of lines
fn read_lines<R: Read>(reader: R) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        result.push(line.to_string());
    }
    result
}
