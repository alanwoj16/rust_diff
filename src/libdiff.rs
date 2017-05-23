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

    convert_to_diffitems(to, from, &mut diffs, );
    
}


pub fn convert_to_diffitems<'a, T: PartialEq + Display>(from: &'a [T],
                                              to: &'a [T],
                                              diffs: &mut Vec<String>){
    let mut i = 0;
    let mut j = 0;
    let mut edit_tracker: Vec<String> = vec![];
    let mut s1 = 1;
    let mut s2 = 1;
    let mut holder = 1;
    let length = diffs.len();
    for edit in diffs{
        
        edit_tracker.push(edit.clone());
        
        if *edit == "s".to_string() {
            //check_diff(&mut edit_tracker, &s1, &s2, &i, &j);
            
            if i > 0 && j > 0{
                check_diff(&mut edit_tracker, &s1, &s2, &i, &j);
                //println!("{} , {}", s1,i);

                //println!("{} , {}",s2,j);
            }
            
            i += 1;
            j += 1;
            s1 = i+1;
            s2 = j+1;
            
        }
        else if *edit == "-".to_string(){
            i += 1;
            if holder == length{
                check_diff(&mut edit_tracker, &s1, &s2, &i, &j);
                //println!("{} , {}", s1,i);
                //println!("{} , {}",s2,j);
            
            }
            
        }

        else if *edit == "+".to_string(){
            j += 1;
            if holder == length{
                check_diff(&mut edit_tracker, &s1, &s2, &i, &j);
                //println!("{} , {}", s1,i);
                //println!("{} , {}",s2,j);

            
            }
            
        }
        //println!("{}",edit);
        
        holder += 1;
       
    }

    //check_diff(&mut edit_tracker);

    //println!("{},{}",i - s1 + 1,i);
    //println!("{},{}",j- s2 + 1,j);


    //for x in edit_tracker{
    //    println!("{}",x);
    //}

}
    
pub fn check_diff(edit_tracker: &mut Vec<String>, s1: &usize,
                                                  s2: &usize,
                                                  i: &usize,
                                                  j: &usize) {


    if !edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()){
        println!{"{},{}d{}",s1,i,j};
        edit_tracker.drain(..);
        

    } else if !edit_tracker.contains(&"-".to_string()) && edit_tracker.contains(&"+".to_string()){
        println!("{}a{},{}",i,s2,j);
        edit_tracker.drain(..);
        
        
    } else if edit_tracker.contains(&"+".to_string()) && edit_tracker.contains(&"-".to_string()){ 
        println!("{},{}c{},{}",s1,i,s2,j);
        edit_tracker.drain(..);
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


