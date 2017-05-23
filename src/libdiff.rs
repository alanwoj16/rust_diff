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

pub fn diff_init<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
                                              from: &'a [T],
                                              to: &'a [T]){
    
    let mut diffs: Vec<String> = vec![];
    make_diffs(table, from, to, from.len(), to.len(), &mut diffs);
    let mut index1 = 0;
    let mut index2 = 0;
    convert_to_diffitems(to, from, &mut index1, &mut index2,  &mut diffs, );
    
}


pub fn convert_to_diffitems<'a, T: PartialEq + Display>(from: &'a [T],
                                              to: &'a [T],
                                              i: &mut usize,
                                              j: &mut usize,
                                              diffs: &mut Vec<String>){

    let mut edit_tracker: Vec<String> = vec![];

    for edit in diffs{

        edit_tracker.push(edit.clone());
        if *edit == "s".to_string(){
            check_diff(&mut edit_tracker);
            if *i > 0 && *j > 0{
                println!("{},{}",i,j);
            }
            
            *i += 1;
            *j += 1;
        }
        else if *edit == "-".to_string(){
            *i += 1;
        }
        else if *edit == "+".to_string(){
            *j +=1;
        }
        //println!("{}",edit);
        //println!("{} , {}", i,j);
    }

    check_diff(&mut edit_tracker);
    println!("{},{}",i,j);


    //for x in edit_tracker{
    //    println!("{}",x);
    //}

}
    
pub fn check_diff(empty: &mut Vec<String>) {


    if !empty.contains(&"+".to_string()) && empty.contains(&"-".to_string()){
        println!{"d"};
        empty.drain(..);
        

    } else if !empty.contains(&"-".to_string()) && empty.contains(&"+".to_string()){
        println!("a");
        empty.drain(..);
        
        
    } else if empty.contains(&"+".to_string()) && empty.contains(&"-".to_string()){ 
        println!("c");
        empty.drain(..);
    }

}

pub fn make_diffs<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
                                              from: &'a [T],
                                              to: &'a [T],
                                              i: usize,
                                              j: usize,
                                              diffs: &mut Vec<String>) {

    if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
        make_diffs(table, from, to, i - 1, j - 1, diffs);
        //diffs.push(format!("s {}", from[i - 1]));
        diffs.push("s".to_string());
    } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
        make_diffs(table, from, to, i, j - 1, diffs);
        //diffs.push(format!("+ {}", to[j - 1]));
        diffs.push("+".to_string());
    } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
        make_diffs(table, from, to, i - 1, j, diffs);
        //diffs.push(format!("- {}", from[i - 1]));
        diffs.push("-".to_string());
    }
}


#[allow(dead_code)]
//pub fn diff<'a, T: PartialEq + Display>(table: &Vec<Vec<usize>>,
//				    from: &'a [T], 
//				    to: &'a [T],
//				    i: usize,
//				    j: usize,
//				    empty: &mut Vec<String>) {
  //  
 //
 //   if i > 0 && j > 0 && from[i - 1] == to[j - 1] {
 //       diff(table, from, to, i - 1, j - 1, empty);
 //       check_diff(empty);
  //      empty.push("n".to_string());
  //  } else if j > 0 && (i == 0 || table[i][j - 1] >= table[i - 1][j]) {
 //       diff(table, from, to, i, j - 1, empty);
 //       empty.push("+".to_string());
  //  } else if i > 0 && (j == 0 || table[i][j - 1] < table[i - 1][j]) {
  //      diff(table, from, to, i - 1, j, empty);
 //       empty.push("-".to_string());
 //  }

  //  if i == from.len() && j == to.len() && !empty.is_empty(){ //if last edit was add or delete
  //      check_diff(empty);
  //  }
//}

//checks to see which diff item to add


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


