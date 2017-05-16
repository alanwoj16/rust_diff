use std::cmp;
use std::fmt::Display;
#[allow(dead_code)]
enum DiffItem<'a, T: 'a + PartialEq> {
    Add { start: usize, lines: &'a [T] },
    Delete { start: usize, len: usize },
    Change {
        start: usize,
        from: &'a [T],
        to: &'a [T],
    },
}

// struct DiffIterator<'a, T: 'a + PartialEq> {
//     items: &'a vec<DiffItem<
// }

// impl<'a, T> Iterator for DiffIterator<'a, T> {
//     type Item = DiffItem<'a, T>;
//     fn next(&mut self) ->Option<T> {

//     }
// }

// fn diff<'a, T>(from: &'a [T], to: &'a [T]) -> DiffIterator<'a, T> { ... }

// fn longest_common_sequence<'a, T: PartialEq> (a: &'a [T], b: &'a [T]) -> &'a [T] {
//     if a.len() <= 0 || b.len() <= 0 {
//         &[]
//     }

// }

pub fn build_lcs_table<'a, T: PartialEq>(from: &'a [T], to: &'a [T]) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = Vec::with_capacity(from.len() + 1);
    // could probably do this with iterators or map()
    for i in 0..from.len() + 1 {
        table.push(Vec::with_capacity(to.len() + 1));
        for _ in 0..to.len() + 1 {
            table[i].push(0);
        }
    }
    for i in 1..from.len() {
        for j in 1..to.len() {
            if from[i - 1] == to[j - 1] {
                table[i][j] = table[i - 1][j - 1] + 1
            } else {
                table[i][j] = cmp::max(table[i][j - 1], table[i - 1][j]);
            }
        }
    }
    table
}


pub fn print_diff<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
                                          from: &'a [T],
                                          to: &'a [T],
                                          i: usize,
                                          j: usize) {
    
    if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
        print_diff(table, from, to, i - 1, j - 1);
        println!(" {}", from[i - 1]);
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        print_diff(table, from, to, i, j - 1);
        println!("+ {}", to[j - 1]);
    } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
        print_diff(table, from, to, i - 1, j);
        println!("- {}", from[i - 1]);
    }
}

#[cfg(test)]
pub mod test {
    use super::{print_diff, build_lcs_table};

    #[test]
    fn test1() {
        let from = vec!["this", "is", "test"];
        let to = vec!["this", "is", "a", "test"];
        let table = build_lcs_table(&from, &to);
        print_diff(&table, &from, &to, from.len(), to.len());
    }
}
