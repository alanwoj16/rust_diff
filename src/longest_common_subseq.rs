use std::cmp;
/// A type alias for a longest common subsequence table
type LCSTable = Vec<Vec<usize>>;

/// Build a longest common subsequence table (necessary for creating the diff)
#[allow(dead_code)]
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