use std::fmt::{Display, Formatter, Error, Debug};

#[derive(Debug, PartialEq, Eq)]
pub enum DiffItem<'a, T: 'a + PartialEq + Debug> {
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

impl<'a, T: 'a + PartialEq + Debug + Display> Display for DiffItem<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            // ADD
            DiffItem::Add {
                start_doc1,
                start_doc2,
                end_doc2,
                lines,
            } => {
                if lines.len() > 1 {
                    writeln!(f, "{}a{},{}", start_doc1, start_doc2, end_doc2).unwrap();
                } else {
                    writeln!(f, "{}a{}", start_doc1, start_doc2).unwrap();

                }
                for line in lines {
                    writeln!(f, "> {}", line).unwrap();
                }
            }
            // DELETE
            DiffItem::Delete {
                start_doc1,
                end_doc1,
                start_doc2,
                lines,
            } => {
                if start_doc1 == end_doc1 {
                    writeln!(f, "{}d{}", start_doc1, start_doc2).unwrap();

                } else {
                    writeln!(f, "{},{}d{}", start_doc1, end_doc1, start_doc2).unwrap();

                }
                for line in lines {
                    writeln!(f, "< {}", line).unwrap();
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
                if from.len() > 1 {
                    writeln!(f, "{},{}c{},{}", start_doc1, end_doc1, start_doc2, end_doc2).unwrap();
                } else {
                    writeln!(f, "{}c{}", start_doc1, start_doc2).unwrap();
                }
                for line in from {
                    writeln!(f, "< {}", line).unwrap();
                }
                writeln!(f, "-----------").unwrap();
                for line in to {
                    writeln!(f, "> {}", line).unwrap();
                }
            }
            DiffItem::Holder => {}
        }
        Ok(())
    }
}