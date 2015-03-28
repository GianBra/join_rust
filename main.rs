use std::io::{BufRead, BufReader};
use std::env;
use std::fs::File;
use std::cmp::Ordering;

pub fn main() {

    let file1 = File::open(&env::args().nth(1).unwrap()).unwrap();
    let file2 = File::open(&env::args().nth(2).unwrap()).unwrap();
    
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    
    let mut iter1 = reader1.lines();
    let mut iter2 = reader2.lines();
    
    let mut opt1 = iter1.next();
    let mut opt2 = iter2.next();
    
    while opt1.is_some() && opt2.is_some()
    {
        let str1 = opt1.clone().unwrap().unwrap();
        let str2 = opt2.clone().unwrap().unwrap();
    
        let line_tok1: Vec<_> = str1.split(" ").collect();
        let line_tok2: Vec<_> = str2.split(" ").collect();

        match line_tok1[0].cmp(line_tok2[0]) {
              Ordering::Equal   => { println!("{} {} {}", line_tok1[0], line_tok1[1], line_tok2[1]);
                                     opt1 = iter1.next(); 
                                     opt2 = iter2.next();}
              Ordering::Less    => { opt1 = iter1.next();}
              Ordering::Greater => { opt2 = iter2.next();}
              }
        
        }
}
