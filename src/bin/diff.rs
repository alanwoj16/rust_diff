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
    let mut prettyprint = false;
    if args.len() == 4 && args[3] == "--pretty-print" {
        prettyprint = true;
    }

    let file_a = File::open(&args[1]).unwrap();
    let file_b = File::open(&args[2]).unwrap();

    let lines_a = read_lines(BufReader::new(file_a));
    let lines_b = read_lines(BufReader::new(file_b));

    let diffs = diff(&lines_a, &lines_b);

    if prettyprint {
        for diff in &diffs {
            pretty_print(stdout(), &lines_a, &diff);
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
