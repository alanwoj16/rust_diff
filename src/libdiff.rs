extern crate colored;
use colored::*;
use std::cmp;
use std::fmt::{Display, Debug};
pub mod diffitem;
use diffitem::DiffItem;

type LCSTable = Vec<Vec<usize>>;

///Initializes diff process. runs make_diffs to get vec of edits (s,+,-)
///Uses convert to diffitems to make diffitems based off vec of edits
///TO DO have diff_init return DiffIterator?
pub fn diff<'a, T>(from: &'a [T], to: &'a [T]) -> Vec<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{
    let table = build_lcs_table(from, to);
    let mut diffs: Vec<String> = vec![];
    make_diffs(&table, from, to, from.len(), to.len(), &mut diffs);
    convert_to_diffitems(from, to, &diffs)
}

//Converts vec of edits to make diffitems. For now prints out results
///TO DO have function return a diffiterator. have check_diff return diffitem,
///and then store it in the diffiterator.
fn convert_to_diffitems<'a, T>(from: &'a [T],
                               to: &'a [T],
                               diffs: &Vec<String>)
                               -> Vec<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{
    //->DiffIterator?{
    let mut result: Vec<DiffItem<'a, T>> = Vec::new();

    let mut ind_from = 0; //index of from slice
    let mut ind_to = 0; //index of to slice
    let mut edit_tracker: Vec<String> = vec![];
    let mut s_from = 1; //index of last same line of from slice
    let mut s_to = 1; //index of last same line of to slice
    let mut num_diffs = 1;
    let diff_length = diffs.len();

    for edit in diffs {

        edit_tracker.push(edit.clone());

        if *edit == "s".to_string() {
            if ind_from > 0 && ind_to > 0 {
                match check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to) {
                    Some(x) => result.push(x),
                    None => {}
                }
            }
            ind_from += 1;
            ind_to += 1;
            s_from = ind_from + 1;
            s_to = ind_to + 1;
        } else if *edit == "-".to_string() {
            ind_from += 1;
            if num_diffs == diff_length {
                match check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to) {
                    Some(x) => result.push(x),
                    None => {}
                }
            }
        } else if *edit == "+".to_string() {
            ind_to += 1;
            if num_diffs == diff_length {
                match check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to) {
                    Some(x) => result.push(x),
                    None => {}
                }
            }
        }
        num_diffs += 1;
    }
    result
}

///Finds out information about diff and converts to diffitem. For now it just prints info
///TO DO make and return DiffItem at end of each if block
fn check_diff<'a, T>(edit_tracker: &mut Vec<String>,
                     s1: usize,
                     s2: usize,
                     i: usize,
                     j: usize,
                     from: &'a [T],
                     to: &'a [T])
                     -> Option<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{

    if !edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Delete {
                        start_doc1: s1,
                        end_doc1: i,
                        start_doc2: s2 - 1,
                        lines: &from[s1 - 1..i],
                    });
    } else if !edit_tracker.contains(&"-".to_string()) && edit_tracker.contains(&"+".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Add {
                        start_doc1: s1 - 1,
                        start_doc2: s2,
                        end_doc2: j + 1,
                        lines: &to[s2 - 1..j],
                    });
    } else if edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Change {
                        start_doc1: s1,
                        start_doc2: s2,
                        end_doc1: i,
                        end_doc2: j,
                        from: &from[s1 - 1..i],
                        to: &to[s2 - 1..j],
                    });
    } else {
        return None;
    }

}

///Builds array with s for same, + for add, - for delete
fn make_diffs<'a, T>(table: &LCSTable,
                     from: &'a [T],
                     to: &'a [T],
                     i: usize,
                     j: usize,
                     diffs: &mut Vec<String>)
    where T: PartialEq + Display + Debug
{

    if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
        make_diffs(table, from, to, i - 1, j - 1, diffs);
        diffs.push("s".to_string());
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        make_diffs(table, from, to, i, j - 1, diffs);
        diffs.push("+".to_string());
    } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
        make_diffs(table, from, to, i - 1, j, diffs);
        diffs.push("-".to_string());
    }
}

/// Build a longest common subsequence table (necessary for creating the diff)
fn build_lcs_table<'a, T: PartialEq>(from: &'a [T], to: &'a [T]) -> LCSTable {
    let mut table: LCSTable = Vec::with_capacity(from.len() + 1);
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
                table[i][j] = table[i - 1][j - 1] + 1;
            } else {
                table[i][j] = cmp::max(table[i][j - 1], table[i - 1][j]);
            }
        }
    }
    table
}

pub fn patch<'a, T>(input: &[T], diff: &DiffItem<T>) -> Vec<T>
    where T: Clone + Debug + PartialEq
{
    let mut changes: Vec<T>;

    match *diff {
        DiffItem::Change {
            start_doc1,
            end_doc1,
            to,
            ..
        } => {
            changes = input[0..start_doc1 - 1].to_vec();
            for i in to {
                changes.push(i.clone());
            }
            for j in end_doc1..input.len() {
                changes.push(input[j].clone());
            }
        }
        DiffItem::Add { start_doc1, lines, .. } => {
            changes = input[0..start_doc1].to_vec();
            for i in lines {
                changes.push(i.clone());
            }
            for j in start_doc1..input.len() {
                changes.push(input[j].clone());
            }
        }
        DiffItem::Delete {
            start_doc1,
            end_doc1,
            ..
        } => {
            changes = input[0..start_doc1 - 1].to_vec();
            for i in end_doc1..input.len() {
                changes.push(input[i].clone());
            }
        }
    }

    changes

}

pub fn pretty_print<'a, String>(original: &'a [String], diff: &DiffItem<String>)
    where String: Clone + Debug + PartialEq + Display
{
    println!("How to make file1 like file 2:");
    match *diff {
        DiffItem::Change {
            start_doc1,
            end_doc1,
            from,
            to,
            ..
        } => {

            for i in 0..start_doc1 - 1 {
                println!("{}", original[i]);
            }
            for j in from {
                println!("{} {}", "-".red(), j.to_string().clone().red());
            }
            for k in to {
                println!("{} {}", "+".green(), k.to_string().clone().green());
            }
            for h in end_doc1..from.len() {
                println!("{}", original[h]);
            }
        }
        DiffItem::Add { start_doc1, lines, .. } => {
            for i in 0..start_doc1 {
                println!("{}", original[i]);
            }
            for j in lines {
                println!("{} {}", "+".green(), j.to_string().clone().green());
            }
            for h in start_doc1..original.len() {
                println!("{}", original[h]);
            }
        }
        DiffItem::Delete {
            start_doc1,
            end_doc1,
            lines,
            ..
        } => {
            for i in 0..start_doc1 - 1 {
                println!("{}", original[i]);
            }
            for j in lines {
                println!("{} {}", "-".red(), j.to_string().clone().red());
            }
            for h in end_doc1..original.len() {
                println!("{}", original[h]);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lcs_table() {
        let a = vec![1, 2, 3];
        let b = vec![1, 5, 3];
        let table = build_lcs_table(&a, &b);
        let expected = vec![vec![0, 0, 0, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
        assert_eq!(table, expected);
    }

    #[test]
    fn test_make_diffs() {
        let a = vec![1, 2, 3];
        let b = vec![1, 5, 3];
        let table = build_lcs_table(&a, &b);
        let mut diffs: Vec<String> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let expected = vec!["s".to_string(), "-".to_string(), "+".to_string(), "s".to_string()];
        assert_eq!(diffs, expected);
    }

    #[test]
    fn test_convert_to_diffitems_change() {
        let a = vec![1, 2, 3];
        let b = vec![1, 5, 3];
        let table = build_lcs_table(&a, &b);
        let mut diffs: Vec<String> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let diffitems = convert_to_diffitems(&a, &b, &diffs);
        let from = [2];
        let to = [5];
        let expected = vec![DiffItem::Change {
                                start_doc1: 2,
                                start_doc2: 2,
                                end_doc1: 2,
                                end_doc2: 2,
                                from: &from,
                                to: &to,
                            }];
        assert_eq!(diffitems, expected);
    }

    #[test]
    fn test_convert_to_diffitems_add_delete() {
        let a = vec![1, 2, 3];
        let b = vec![1, 3, 4];
        let table = build_lcs_table(&a, &b);
        let mut diffs: Vec<String> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let diffitems = convert_to_diffitems(&a, &b, &diffs);
        let del = [2];
        let add = [4];
        let expected = vec![DiffItem::Delete {
                                start_doc1: 2,
                                end_doc1: 2,
                                start_doc2: 1,
                                lines: &del,
                            },
                            DiffItem::Add {
                                start_doc1: 3,
                                start_doc2: 3,
                                end_doc2: 4,
                                lines: &add,
                            }];
        assert_eq!(diffitems, expected);
    }

    #[test]
    fn test_diff_strings() {
        let a = vec!["1", "2", "3"];
        let b = vec!["1", "3", "4"];
        let diffitems = diff(&a, &b);
        let del = ["2"];
        let add = ["4"];
        let expected = vec![DiffItem::Delete {
                                start_doc1: 2,
                                end_doc1: 2,
                                start_doc2: 1,
                                lines: &del,
                            },
                            DiffItem::Add {
                                start_doc1: 3,
                                start_doc2: 3,
                                end_doc2: 4,
                                lines: &add,
                            }];
        assert_eq!(diffitems, expected);
    }

    #[test]
    fn test_patch() {
        let a = vec!["1", "2", "3"];
        let b = vec!["1", "3", "4"];
        let diffitems = diff(&a, &b);
        let mut patched = patch(&a, &diffitems[0]);
        assert_eq!(patched, vec!["1", "3"]);

        patched = patch(&a, &diffitems[1]);
        assert_eq!(patched, vec!["1", "2", "3", "4"]);
    }

    #[test]
    fn test_diff_longer() {
        let a = "the quick brown fox jumped over the lazy dog";
        let b = "thequick brown fox juumped over and lazy dog dog";
        let del = " ";
        let add = "u";
        let from = "the";
        let to = "and";
        let add2 = " dog";
        let diffitems = diff(&a.as_bytes(), &b.as_bytes());
        let expected = vec![DiffItem::Delete {
                                start_doc1: 4,
                                end_doc1: 4,
                                start_doc2: 3,
                                lines: &del.as_bytes(),
                            },
                            DiffItem::Add {
                                start_doc1: 21,
                                start_doc2: 21,
                                end_doc2: 22,
                                lines: &add.as_bytes(),
                            },
                            DiffItem::Change {
                                start_doc1: 33,
                                start_doc2: 33,
                                end_doc1: 35,
                                end_doc2: 35,
                                from: &from.as_bytes(),
                                to: &to.as_bytes(),
                            },
                            DiffItem::Add {
                                start_doc1: 40,
                                start_doc2: 41,
                                end_doc2: 45,
                                lines: &add2.as_bytes(),
                            }];
        assert_eq!(diffitems, expected);
    }
}