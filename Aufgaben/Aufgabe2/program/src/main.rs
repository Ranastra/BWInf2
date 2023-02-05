use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let (n, lowest, all_slices) = read_input(1);
}

fn read_input(number_of_test: i32) -> (i32, i32, HashMap::<[i32; 2], i32>) {
    let s = format!("../testcases/bsp{}.txt", number_of_test.to_string());
    let file = File::open(s).unwrap();
    let mut all_slices = HashMap::<[i32; 2], i32>::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut lowest = n;
    for line in lines {
        let line = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let n1 = i32::from_str(line2.next().unwrap()).unwrap();
        let n2 = i32::from_str(line2.next().unwrap()).unwrap();
        if n1 < lowest {lowest = n1;}
        let nums: [i32; 2] = [n1, n2];
        *all_slices.entry(nums).or_insert(0) += 1;
    };
    (n, lowest, all_slices)
}
