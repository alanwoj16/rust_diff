/// diff.rs
///
/// A command line diff utility
///
/// Given two text files, prints out the differences between them
/// and edit scripts to show how to make the first look like the second
///
/// Usage: diff from.txt to.txt
///

extern crate diff;
use diff::{diff, pretty_print};
use std::io::{Read, BufReader, BufRead, stdout};
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        panic!("diff requires two paths to text files as arguments");
    }
    let file_a = File::open(&args[1]).unwrap();
    let file_b = File::open(&args[2]).unwrap();

    let lines_a = read_lines(BufReader::new(file_a));
    let lines_b = read_lines(BufReader::new(file_b));

    let diffs = diff(&lines_a, &lines_b);

    if args.len() == 4 && args[3] == "--steps" {
        let mut i = 0;
        for diff in &diffs {
            i += 1;
            println!("\nEdit {}:", i);
            pretty_print(stdout(), &lines_a, diff);
        }
    } else {
        for diff in &diffs {
            print!("{}", *diff);
        }
    }

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
