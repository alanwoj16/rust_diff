use std::fmt::{Display, Debug};
use diffitem::DiffItem;


type LCSTable = Vec<Vec<usize>>;

/// Represents the status of a single item in the sequence.
/// Used as an intermediate representation when calculating the diff
///
/// Example:
///  from: A B C
///  to: A D C
///
///  A -> Same
///  B -> Delete
///  D -> Add
///  C -> Same
///
#[derive(Debug, PartialEq, Clone)]
pub enum EditFlags {
    Add,
    Delete,
    Same,
}
impl ToString for EditFlags {
    fn to_string(&self) -> String {
        match self {
            &EditFlags::Add => "+".to_string(),
            &EditFlags::Delete => "-".to_string(),
            &EditFlags::Same => "s".to_string(),
        }
    }
}

/// Helper function that converts a Vec<EditFlags> to a Vec<DiffItem>
/// Merges consecutive adds/deletes into a single add, delete, or Change
pub fn convert_to_diffitems<'a, T>(from: &'a [T],
                                   to: &'a [T],
                                   diffs: &Vec<EditFlags>)
                                   -> Vec<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{
    let mut result: Vec<DiffItem<'a, T>> = Vec::new();

    let mut ind_from = 0; //index of from slice
    let mut ind_to = 0; //index of to slice
    let mut edit_tracker: Vec<EditFlags> = vec![];
    let mut s_from = 1; //index of last same line of from slice
    let mut s_to = 1; //index of last same line of to slice
    let mut num_diffs = 1;
    let diff_length = diffs.len();

    for edit in diffs {

        edit_tracker.push(edit.clone());

        match edit {
            &EditFlags::Same => {
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
            }
            &EditFlags::Delete => {
                ind_from += 1;
                if num_diffs == diff_length {
                    match check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to) {
                        Some(x) => result.push(x),
                        None => {}
                    }
                }
            }
            &EditFlags::Add => {
                ind_to += 1;
                if num_diffs == diff_length {
                    match check_diff(&mut edit_tracker, s_from, s_to, ind_from, ind_to, from, to) {
                        Some(x) => result.push(x),
                        None => {}
                    }
                }
            }
        }
        num_diffs += 1;
    }
    result
}

/// Helper function for convert_to_diffitems
/// Finds a single sequence of consecutive edits and merges them into one DiffItem
fn check_diff<'a, T>(edit_tracker: &mut Vec<EditFlags>,
                     s1: usize,
                     s2: usize,
                     i: usize,
                     j: usize,
                     from: &'a [T],
                     to: &'a [T])
                     -> Option<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{

    if !edit_tracker.contains(&EditFlags::Add) && edit_tracker.contains(&EditFlags::Delete) {
        edit_tracker.drain(..);
        return Some(DiffItem::Delete {
                        start_from: s1,
                        end_from: i,
                        start_to: s2 - 1,
                        items: &from[s1 - 1..i],
                    });
    } else if !edit_tracker.contains(&EditFlags::Delete) && edit_tracker.contains(&EditFlags::Add) {
        edit_tracker.drain(..);
        return Some(DiffItem::Add {
                        start_from: s1 - 1,
                        start_to: s2,
                        end_to: j + 1,
                        items: &to[s2 - 1..j],
                    });
    } else if edit_tracker.contains(&EditFlags::Add) && edit_tracker.contains(&EditFlags::Delete) {
        edit_tracker.drain(..);
        return Some(DiffItem::Change {
                        start_from: s1,
                        start_to: s2,
                        end_from: i,
                        end_to: j,
                        from: &from[s1 - 1..i],
                        to: &to[s2 - 1..j],
                    });
    } else {
        return None;
    }

}

/// Find all the edits necessary to make "from" match "to"
/// Every edited item is representated by an EditFlag.
/// There may be multiple consecutive edits, use convert_to_diffitems to merge them.
pub fn make_diffs<'a, T>(table: &LCSTable,
                         from: &'a [T],
                         to: &'a [T],
                         i: usize,
                         j: usize,
                         diffs: &mut Vec<EditFlags>)
    where T: PartialEq + Display + Debug
{

    if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
        make_diffs(table, from, to, i - 1, j - 1, diffs);
        diffs.push(EditFlags::Same);
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        make_diffs(table, from, to, i, j - 1, diffs);
        diffs.push(EditFlags::Add);
    } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
        make_diffs(table, from, to, i - 1, j, diffs);
        diffs.push(EditFlags::Delete);
    }
}