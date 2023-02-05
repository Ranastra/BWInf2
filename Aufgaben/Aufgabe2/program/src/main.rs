use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let file = File::open("../testcases/bsp1.txt").unwrap();
    let mut set = HashSet::<[i32; 2]>::new();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let n = lines.next();
    println!("{}", n);
    for line in lines {
        let line: String = line.unwrap();
        println!("{}", line);
    }
}
