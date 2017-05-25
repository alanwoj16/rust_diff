use std::cmp;
use std::fmt::{Display, Debug};
pub mod diffitem;
use diffitem::DiffItem;

type LCSTable = Vec<Vec<usize>>;

///Initializes diff process. runs make_diffs to get vec of edits (s,+,-)
///Uses convert to diffitems to make diffitems based off vec of edits
///TO DO have diff_init return DiffIterator?
pub fn diff<'a, T: PartialEq + Display + Debug>(from: &'a [T],
                                                to: &'a [T])
                                                -> Vec<DiffItem<'a, T>> {
    let table = build_lcs_table(from, to);
    let mut diffs: Vec<String> = vec![];
    make_diffs(&table, from, to, from.len(), to.len(), &mut diffs);

    convert_to_diffitems(to, from, &mut diffs)
}

//Converts vec of edits to make diffitems. For now prints out results
///TO DO have function return a diffiterator. have check_diff return diffitem,
///and then store it in the diffiterator.
fn convert_to_diffitems<'a, T: PartialEq + Display + Debug>(from: &'a [T],
                                                            to: &'a [T],
                                                            diffs: &mut Vec<String>)
                                                            -> Vec<DiffItem<'a, T>> {
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
                result.push(check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to).unwrap());
            }
            ind_from += 1;
            ind_to += 1;
            s_from = ind_from + 1;
            s_to = ind_to + 1;
        } else if *edit == "-".to_string() {
            ind_from += 1;
            if num_diffs == diff_length {
                result.push(check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to).unwrap());
            }
        } else if *edit == "+".to_string() {
            ind_to += 1;
            if num_diffs == diff_length {
                result.push(check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to).unwrap());
            }
        }
        num_diffs += 1;
    }
    result
}

///Finds out information about diff and converts to diffitem. For now it just prints info
///TO DO make and return DiffItem at end of each if block
fn check_diff<'a, T: PartialEq + Display + Debug>(edit_tracker: &mut Vec<String>,
                                                  s1: usize,
                                                  s2: usize,
                                                  i: usize,
                                                  j: usize,
                                                  from: &'a [T],
                                                  to: &'a [T])
                                                  -> Option<DiffItem<'a, T>> {

    if !edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Delete {
                        start_doc1: s1,
                        end_doc1: i,
                        start_doc2: s2 - 1,
                        lines: &to[s1 - 1..i],
                    });
    } else if !edit_tracker.contains(&"-".to_string()) && edit_tracker.contains(&"+".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Add {
                        start_doc1: s1 - 1,
                        start_doc2: s2,
                        end_doc2: j + 1,
                        lines: &from[s2 - 1..j],
                    });
    } else if edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()) {
        edit_tracker.drain(..);
        return Some(DiffItem::Change {
                        start_doc1: s1,
                        start_doc2: s2,
                        end_doc1: i,
                        end_doc2: j,
                        from: &from[s1 - 1..j],
                        to: &to[s2 - 1..i],
                    });
    } else {
        return Some(DiffItem::Holder);
    }

}

///Builds array with s for same, + for add, - for delete
fn make_diffs<'a, T: PartialEq + Display + Debug>(table: &LCSTable,
                                                  from: &'a [T],
                                                  to: &'a [T],
                                                  i: usize,
                                                  j: usize,
                                                  diffs: &mut Vec<String>) {

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

// Prints a diff given two slices and the corresponding LCS table
pub fn print_diff<'a, T: PartialEq + Display + Debug>(table: &LCSTable,
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
