use std::fmt::{Display, Formatter, Error, Debug};

/// The DiffItem enum. Represents an edit action (either Add, Delete, or Change)
/// Contains the necessary information to make the corresponding change to a slice.
#[derive(Debug, PartialEq, Eq)]
pub enum DiffItem<'a, T: 'a>
    where T: PartialEq + Debug
{
    /// Represents an insertion edit
    Add {
        /// The index at which to insert the new items in the "from" slice
        start_from: usize,
        /// The starting index of the new items in the "to" slice
        start_to: usize,
        /// The ending index of the new items in the "to" slice
        end_to: usize,
        /// The items to be inserted
        items: &'a [T],
    },
    /// Represents a deletion edit
    Delete {
        /// The starting index of the items to delete from the "from" slice
        start_from: usize,
        /// The ending index of the items to delete from the "from" slice
        end_from: usize,
        /// The index in "to" that corresponds to "start_from"
        start_to: usize,
        /// The items to be deleted
        items: &'a [T],
    },
    /// Represents a change edit
    Change {
        /// The starting index of the items to Change from the "from" slice
        start_from: usize,
        /// The ending index of the items to Change from the "from" slice
        end_from: usize,
        /// The starting index of the items to Change from the "to" slice
        start_to: usize,
        /// The ending index of the items to Change from the "to" slice
        end_to: usize,
        /// The lines that will be Changed in "from"
        from: &'a [T],
        /// The change lines from "to"
        to: &'a [T],
    },
}

impl<'a, T: 'a> Display for DiffItem<'a, T>
    where T: PartialEq + Debug + Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            // ADD
            DiffItem::Add {
                start_from,
                start_to,
                end_to,
                items,
            } => {
                if items.len() > 1 {
                    writeln!(f, "{}a{},{}", start_from, start_to, end_to).unwrap();
                } else {
                    writeln!(f, "{}a{}", start_from, start_to).unwrap();

                }
                for item in items {
                    writeln!(f, "> {}", item).unwrap();
                }
            }
            // DELETE
            DiffItem::Delete {
                start_from,
                end_from,
                start_to,
                items,
            } => {
                if start_from == end_from {
                    writeln!(f, "{}d{}", start_from, start_to).unwrap();

                } else {
                    writeln!(f, "{},{}d{}", start_from, end_from, start_to).unwrap();

                }
                for item in items {
                    writeln!(f, "< {}", item).unwrap();
                }
            }
            // Change
            DiffItem::Change {
                start_from,
                start_to,
                end_from,
                end_to,
                from,
                to,
            } => {
                if from.len() > 1 {
                    writeln!(f, "{},{}c{},{}", start_from, end_from, start_to, end_to).unwrap();
                } else {
                    writeln!(f, "{}c{}", start_from, start_to).unwrap();
                }
                for item in from {
                    writeln!(f, "< {}", item).unwrap();
                }
                writeln!(f, "-----------").unwrap();
                for item in to {
                    writeln!(f, "> {}", item).unwrap();
                }
            }
        }
        Ok(())
    }
}
