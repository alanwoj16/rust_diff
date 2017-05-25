use std::cmp;
use std::fmt::Display;
pub mod diffitem;
use diffitem::DiffItem;

type LCSTable = Vec<Vec<usize>>;

///Initializes diff process. runs make_diffs to get vec of edits (s,+,-)
///Uses convert to diffitems to make diffitems based off vec of edits
///TO DO have diff_init return DiffIterator?
pub fn diff<'a, T: PartialEq + Display>(from: &'a [T], to: &'a [T]) -> Vec<DiffItem<'a, T>> {
    let table = build_lcs_table(from, to);
    let mut diffs: Vec<String> = vec![];
    make_diffs(&table, from, to, from.len(), to.len(), &mut diffs);

    convert_to_diffitems(to, from, &mut diffs)
}

//Converts vec of edits to make diffitems. For now prints out results
///TO DO have function return a diffiterator. have check_diff return diffitem,
///and then store it in the diffiterator.
pub fn convert_to_diffitems<'a, T: PartialEq + Display>(from: &'a [T],
                                                        to: &'a [T],
                                                        diffs: &mut Vec<String>)
                                                        -> Vec<DiffItem<'a, T>> {
    //->DiffIterator?{
    let mut result: Vec<DiffItem<'a, T>> = Vec::new();

    let mut i = 0;
    let mut j = 0;
    let mut edit_tracker: Vec<String> = vec![];
    let mut s1 = 1;
    let mut s2 = 1;
    let mut holder = 1;
    let length = diffs.len();
    for edit in diffs {

        edit_tracker.push(edit.clone());

        if *edit == "s".to_string() {
            if i > 0 && j > 0 {
                result.push(check_diff(&mut edit_tracker, s1, s2, i, j, from, to).unwrap());
            }
            i += 1;
            j += 1;
            s1 = i + 1;
            s2 = j + 1;

        } else if *edit == "-".to_string() {
            i += 1;
            if holder == length {
                result.push(check_diff(&mut edit_tracker, s1, s2, i, j, from, to).unwrap());
            }
        } else if *edit == "+".to_string() {
            j += 1;
            if holder == length {
                result.push(check_diff(&mut edit_tracker, s1, s2, i, j, from, to).unwrap());
            }
        }
        holder += 1;
    }
    result
}

///Finds out information about diff and converts to diffitem. For now it just prints info
///TO DO make and return DiffItem at end of each if block
fn check_diff<'a, T: PartialEq + Display>(edit_tracker: &mut Vec<String>,
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
pub fn make_diffs<'a, T: PartialEq + Display>(table: &LCSTable,
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
pub fn build_lcs_table<'a, T: PartialEq>(from: &'a [T], to: &'a [T]) -> LCSTable {
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
pub fn print_diff<'a, T: PartialEq + Display>(table: &LCSTable,
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
