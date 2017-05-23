use std::cmp;
use std::fmt::Display;
#[allow(dead_code)]
pub enum DiffItem<'a, T: 'a + PartialEq> {
    Holder,
    Add { 
        start_doc1: usize, 
    	start_doc2: usize,
	    end_doc2: usize,
	    lines: &'a [T] },
    Delete { 
        start_doc1: usize,
        end_doc1: usize,
	    start_doc2: usize,
	    lines: &'a [T] },
    Change {
        start_doc1: usize,
	    end_doc1: usize,
	    start_doc2: usize,
	    end_doc2: usize,
        from: &'a [T],
        to: &'a [T],
    },
}



#[allow(dead_code)]
struct DiffIterator<'a, T: 'a + PartialEq> {
    
    items: &'a Vec<DiffItem<'a,T>>,
}

#[allow(dead_code)]
impl <'a, T: 'a + PartialEq> DiffIterator<'a, T>{

    fn new_from_list(diffitems: &'a Vec<DiffItem<'a,T>>) -> Self{

        DiffIterator {items: diffitems}
        
    }
}

// impl<'a, T> Iterator for DiffIterator<'a, T> {
//     type Item = DiffItem<'a, T>;
//     fn next(&mut self) ->Option<T> {

//     }
// }

#[allow(dead_code)]
pub fn diff<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
				    from: &'a [T], 
				    to: &'a [T],
				    i: usize,
				    j: usize,
				    empty: &mut Vec<String>) {
    
 
    if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
        diff(table, from, to, i - 1, j - 1, empty);
        check_diff(empty);
        empty.push("n".to_string());
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        diff(table, from, to, i, j - 1, empty);
        empty.push("+".to_string());
    } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
        diff(table, from, to, i - 1, j, empty);
        empty.push("-".to_string());
    }

    if i == from.len() && j == to.len() && !empty.is_empty(){ //if last edit was add or delete
        check_diff(empty);
    }
}

///checks to see which diff item to add
pub fn check_diff(empty: &mut Vec<String>) -> DiffItem<T> {

    let vec = vec!["filler"];

    if !empty.contains(&"+".to_string()) && empty.contains(&"-".to_string()){
        let delete = DiffItem::Delete {start_doc1: 0,
                                       end_doc1: 0,
                                       start_doc2: 0,
                                       lines: &vec[..]};
        empty.drain(..);
        delete

    } else if !empty.contains(&"-".to_string()) && empty.contains(&"+".to_string()){
        let add = DiffItem::Add{start_doc1: 0, 
                                start_doc2: 0,
                                end_doc2: 0,
                                lines: &vec[..]};
        empty.drain(..);
        add
        
    } else if empty.contains(&"+".to_string()) && empty.contains(&"-".to_string()){ 
        let change = DiffItem::Change{start_doc1: 0,
                                      end_doc1: 0,
                                      start_doc2: 0,
                                      end_doc2: 0,
                                      from: &vec[..],
                                      to: &vec[..]};
        empty.drain(..);
        change

    }

}

/// Build a longest common subsequence table (necessary for creating the diff)
pub fn build_lcs_table<'a, T: PartialEq>(from: &'a [T], to: &'a [T]) -> Vec<Vec<usize>> {
    let mut table: Vec<Vec<usize>> = Vec::with_capacity(from.len() + 1);
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
pub fn print_diff<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
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


