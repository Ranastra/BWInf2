use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_data(file_path: &str) -> Vec<(i32, i32)> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut first_line = true;

    for line in reader.lines() {
        let line = line.unwrap();
        if first_line {
            first_line = false;
            continue;
        }
        let parts: Vec<&str> = line.split(", ").collect();
        let num1: i32 = parts[0].parse().unwrap();
        let num2: i32 = parts[1].parse().unwrap();
        data.push((num1, num2));
    }

    data
}

let data = read_data("testcases/bsp1");
