extern crate libdiff;
use libdiff::{build_lcs_table, print_diff};

fn main() {  
  let from = vec!["this", "is", "test"];
        let to = vec!["this", "is", "a", "test"];
        let table = build_lcs_table(&from, &to);
        print_diff(&table, &from, &to, from.len(), to.len());
}
