use std::io::{BufRead, Read, BufReader};
use std::env;
use std::fs::File;
use std::path::Path;

pub fn main() {

    let file1 = File::open(&env::args().nth(1).unwrap()).unwrap();
    let file2 = File::open(&env::args().nth(2).unwrap()).unwrap();
    
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    
    let mut iter1 = reader1.lines();
    let mut iter2 = reader2.lines();
    
    let mut opt1 = iter1.next();
    let mut opt2 = iter2.next();
    
    
    while opt1.is_some() && opt2.is_some() {
    
        let str1 = opt1.unwrap().unwrap();
        let str2 = opt2.unwrap().unwrap();
    
        let line_tok1: Vec<_> = str1.split(" ").collect();
        let line_tok2: Vec<_> = str2.split(" ").collect();
        
        //println!("{} {}", line_tok1[0], line_tok1[1]); // OCIO!
        
        if line_tok1[0] == line_tok2[0] {
            println!("{} {} {}", line_tok1[0], line_tok1[1], line_tok2[1]);
            
        } //else if line_tok1[0] > line_tok2[0] {
            //opt2 = iter2.next();
        //}
        opt1 = iter1.next();
        opt2 = iter2.next();
    }

    //for line in reader1.lines() {
    //    println!("1 {}", line.unwrap());
    //}

}
