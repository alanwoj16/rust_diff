use std::fmt::{Display, Formatter, Error};

pub enum DiffItem<'a, T: 'a + PartialEq> {
    Holder,
    Add {
        start_doc1: usize,
        start_doc2: usize,
        end_doc2: usize,
        lines: &'a [T],
    },
    Delete {
        start_doc1: usize,
        end_doc1: usize,
        start_doc2: usize,
        lines: &'a [T],
    },
    Change {
        start_doc1: usize,
        end_doc1: usize,
        start_doc2: usize,
        end_doc2: usize,
        from: &'a [T],
        to: &'a [T],
    },
}

impl<'a, T: 'a + PartialEq + Display> Display for DiffItem<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            // ADD
            DiffItem::Add {
                start_doc1,
                start_doc2,
                end_doc2,
                lines,
            } => {
                write!(f, "{}a{},{}", start_doc1, start_doc2, end_doc2).unwrap();
                for line in lines {
                    write!(f, "> {}", line).unwrap();
                }
            }
            // DELETE
            DiffItem::Delete {
                start_doc1,
                end_doc1,
                start_doc2,
                lines,
            } => {
                write!(f, "{},{}d{}", start_doc2, start_doc1, end_doc1).unwrap();
                for line in lines {
                    write!(f, "< {}", line).unwrap();
                }
            }
            // CHANGE
            DiffItem::Change {
                start_doc1,
                start_doc2,
                end_doc1,
                end_doc2,
                from,
                to,
            } => {
                write!(f, "{},{}c{},{}", start_doc1, end_doc1, start_doc2, end_doc2).unwrap();
                for line in from {
                    write!(f, "< {}", line).unwrap();
                }
                write!(f, "-----------").unwrap();
                for line in to {
                    write!(f, "> {}", line).unwrap();
                }
            }
            DiffItem::Holder => {}
        }
        Ok(())
    }
}