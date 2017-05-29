/// libdiff.rs
///
/// A library for calculating the diff of two sequences
///

extern crate colored;
use colored::*;
use std::fmt::{Display, Debug};
use std::io::Write;

mod diffhelpers;
use diffhelpers::*;

mod longest_common_subseq;
use longest_common_subseq::build_lcs_table;

mod diffitem;
pub use diffitem::DiffItem;

/// Calculate a diff. Takes in two slices and returns a Vec<DiffItem>
/// containing the changes necessary to make "from" look like "to"
///
/// # Example
/// ```
/// use diff::diff;
///
/// let from = vec!["this", "is", "an", "example"];
/// let to = vec!["this", "is", "another", "example"];
/// let diffs = diff(&from, &to);
/// ```
pub fn diff<'a, T>(from: &'a [T], to: &'a [T]) -> Vec<DiffItem<'a, T>>
    where T: PartialEq + Display + Debug
{
    let table = build_lcs_table(from, to);
    let mut diffs: Vec<_> = vec![];
    make_diffs(&table, from, to, from.len(), to.len(), &mut diffs);
    convert_to_diffitems(from, to, &diffs)
}

/// Applies an edit represented by a DiffItem to a slice.
///
/// # Example
/// ```
/// use diff::{diff, patch};
///
/// let from = vec!["this", "is", "an", "example"];
/// let to = vec!["this", "is", "another", "example"];
///
/// let changes = diff(&from, &to);
///
/// // Apply the first edit:
/// let patched = patch(&from, &changes[0]);
/// ```
pub fn patch<'a, T>(input: &[T], diff: &DiffItem<'a, T>) -> Vec<T>
    where T: Clone + Debug + PartialEq + Display
{
    let mut changes: Vec<T>;

    match *diff {
        DiffItem::Replace {
            start_from,
            end_from,
            to,
            ..
        } => {
            changes = input[0..start_from - 1].to_vec();
            for i in to {
                changes.push(i.clone());
            }
            for j in end_from..input.len() {
                changes.push(input[j].clone());
            }
        }
        DiffItem::Add { start_from, items, .. } => {
            changes = input[0..start_from].to_vec();
            for i in items {
                changes.push(i.clone());
            }
            for j in start_from..input.len() {
                changes.push(input[j].clone());
            }
        }
        DiffItem::Delete {
            start_from,
            end_from,
            ..
        } => {
            changes = input[0..start_from - 1].to_vec();
            for i in end_from..input.len() {
                changes.push(input[i].clone());
            }
        }
    }

    changes
}

/// Prints a colored representation of how to apply an edit to a sequence
///
/// # Example
/// ```
/// use diff::{diff, pretty_print};
/// use std::io::stdout;
///
/// let from = vec!["this", "is", "an", "example"];
/// let to = vec!["this", "is", "another", "example"];
///
/// let changes = diff(&from, &to);
///
/// pretty_print(stdout(), &from, &changes[0]);
/// ```
pub fn pretty_print<'a, T, W>(mut writer: W, original: &'a [T], diff: &DiffItem<'a, T>)
    where T: Clone + Debug + PartialEq + Display,
          W: Write
{
    println!("How to make file 1 like file 2:");
    match *diff {
        DiffItem::Replace {
            start_from,
            end_from,
            from,
            to,
            ..
        } => {

            for i in 0..start_from - 1 {
                writeln!(writer, "{}", original[i]).unwrap();
            }
            for j in from {
                writeln!(writer,
                         "{} {}",
                         EditFlags::Delete.to_string().red(),
                         j.to_string().clone().red())
                        .unwrap();
            }
            for k in to {
                writeln!(writer,
                         "{} {}",
                         EditFlags::Add.to_string().green(),
                         k.to_string().clone().green())
                        .unwrap();
            }
            for h in end_from..from.len() {
                writeln!(writer, "{}", original[h]).unwrap();
            }
        }
        DiffItem::Add { start_from, items, .. } => {
            for i in 0..start_from {
                writeln!(writer, "{}", original[i]).unwrap();
            }
            for j in items {
                writeln!(writer,
                         "{} {}",
                         EditFlags::Add.to_string().green(),
                         j.to_string().clone().green())
                        .unwrap();
            }
            for h in start_from..original.len() {
                writeln!(writer, "{}", original[h]).unwrap();
            }
        }
        DiffItem::Delete {
            start_from,
            end_from,
            items,
            ..
        } => {
            for i in 0..start_from - 1 {
                writeln!(writer, "{}", original[i]).unwrap();
            }
            for j in items {
                writeln!(writer,
                         "{} {}",
                         EditFlags::Delete.to_string().red(),
                         j.to_string().clone().red())
                        .unwrap();
            }
            for h in end_from..original.len() {
                writeln!(writer, "{}", original[h]).unwrap();
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
        let mut diffs: Vec<_> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let expected = vec![EditFlags::Same, EditFlags::Delete, EditFlags::Add, EditFlags::Same];
        assert_eq!(diffs, expected);
    }

    #[test]
    fn test_convert_to_diffitems_replace() {
        let a = vec![1, 2, 3];
        let b = vec![1, 5, 3];
        let table = build_lcs_table(&a, &b);
        let mut diffs: Vec<_> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let diffitems = convert_to_diffitems(&a, &b, &diffs);
        let from = [2];
        let to = [5];
        let expected = vec![DiffItem::Replace {
                                start_from: 2,
                                start_to: 2,
                                end_from: 2,
                                end_to: 2,
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
        let mut diffs: Vec<_> = vec![];
        make_diffs(&table, &a, &b, a.len(), b.len(), &mut diffs);
        let diffitems = convert_to_diffitems(&a, &b, &diffs);
        let del = [2];
        let add = [4];
        let expected = vec![DiffItem::Delete {
                                start_from: 2,
                                end_from: 2,
                                start_to: 1,
                                items: &del,
                            },
                            DiffItem::Add {
                                start_from: 3,
                                start_to: 3,
                                end_to: 4,
                                items: &add,
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
                                start_from: 2,
                                end_from: 2,
                                start_to: 1,
                                items: &del,
                            },
                            DiffItem::Add {
                                start_from: 3,
                                start_to: 3,
                                end_to: 4,
                                items: &add,
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
                                start_from: 4,
                                end_from: 4,
                                start_to: 3,
                                items: &del.as_bytes(),
                            },
                            DiffItem::Add {
                                start_from: 21,
                                start_to: 21,
                                end_to: 22,
                                items: &add.as_bytes(),
                            },
                            DiffItem::Replace {
                                start_from: 33,
                                start_to: 33,
                                end_from: 35,
                                end_to: 35,
                                from: &from.as_bytes(),
                                to: &to.as_bytes(),
                            },
                            DiffItem::Add {
                                start_from: 40,
                                start_to: 41,
                                end_to: 45,
                                items: &add2.as_bytes(),
                            }];
        assert_eq!(diffitems, expected);
    }
}