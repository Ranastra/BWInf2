use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use std::time::{Duration, Instant};

const PRINT_TIME_ALL: bool = true;
const PRINT_TIME_TOTAL: bool = true;
const RUN_WITH_CARGO: bool = false;

fn main() {
    if RUN_WITH_CARGO {
        run_all();
    } else {
        // get arguments from command line
        let args: Vec<String> = env::args().collect();
        let file_name = &args[1];
        if file_name == "all" {
            run_all();
        } else {
            run_one(file_name);
        }
    }
}

fn run_all() {
    let start_time: Instant = Instant::now();
    for i in 1..8 {
        let filename: String = format!("bsp{}.txt", i);
        run_one(&filename);
    }
    if PRINT_TIME_TOTAL {
        let duration: Duration = start_time.elapsed();
        println!("Total time for all tests: {:?}", duration);
    }
}

fn run_one(filename: &str) {
    println!("{}", filename);
    let start_time: Instant = Instant::now();
    let path_to_file: String;
    if RUN_WITH_CARGO {
        path_to_file = format!("../testcases/{}", filename);
    } else {
        path_to_file = format!("testcases/{}", filename);
    }
    let (lowest, lowest_second, all_slices, n) = read_input(&path_to_file);
    solve(lowest, lowest_second, all_slices, &filename);
    if PRINT_TIME_ALL {
        let timedelta: Duration = start_time.elapsed();
        println!("Time per test: {:?}", timedelta);
        let factor: i64 = (timedelta.as_nanos() as i64) / n;
        println!("number of slices: {}", n);
        println!("Time per slice in nanosecs: {}", factor);
    }
}

fn solve(
    lowest: i32,
    lowest_second: HashSet<i32>,
    mut all_slices: HashMap<[i32; 2], i32>,
    path_to_file: &str,
) {
    let mut start_slices: Vec<[i32; 2]> = Vec::new(); // change all_slices to mut
    for (key, value) in all_slices.iter() {
        if *value >= lowest && (lowest_second.contains(&key[0]) || lowest_second.contains(&key[1]))
        {
            start_slices.push(key.clone());
        }
    }
    let mut found_solution: bool = false;
    while !start_slices.is_empty() {
        let start: [i32; 2] = start_slices.pop().unwrap();
        let mut x: i32 = start[0];
        let mut y: i32 = start[1];
        let mut z: i32 = lowest;
        modify_hash_map(&mut all_slices, start, -lowest); //t
        let mut stack: Vec<i32> = Vec::new();
        let mut step_backward: bool = false;
        order(&mut x, &mut y, &mut z);
        let mut state: i32;
        let mut success: bool = true;
        while !all_slices.is_empty() || step_backward {
            if step_backward {
                order(&mut x, &mut y, &mut z);
                if stack.is_empty() {
                    success = false;
                    break;
                }
                state = stack.pop().unwrap();
                let slice = back_backtrack(state, &mut x, &mut y, &mut z);
                modify_hash_map(&mut all_slices, slice, 1); //t
                if state == 3 {
                    continue;
                } else {
                    step_backward = false;
                }
            } else {
                state = 0;
            }
            order(&mut x, &mut y, &mut z);
            let arr: [bool; 3] = [
                all_slices.contains_key(&[x, y]),
                all_slices.contains_key(&[x, z]),
                all_slices.contains_key(&[y, z]),
            ];
            if state == 0 {
                if !arr[0] {
                    if !arr[1] {
                        if !arr[2] {
                            step_backward = true;
                            continue;
                        } else {
                            stack.push(3);
                            x += 1;
                            modify_hash_map(&mut all_slices, [y, z], -1);
                        }
                    } else {
                        stack.push(2);
                        y += 1;
                        modify_hash_map(&mut all_slices, [x, z], -1);
                    }
                } else {
                    stack.push(1);
                    z += 1;
                    modify_hash_map(&mut all_slices, [x, y], -1);
                }
            } else if state == 1 {
                if !arr[1] {
                    if !arr[2] {
                        step_backward = true;
                        continue;
                    } else {
                        stack.push(3);
                        x += 1;
                        modify_hash_map(&mut all_slices, [y, z], -1);
                    }
                } else {
                    stack.push(2);
                    y += 1;
                    modify_hash_map(&mut all_slices, [x, z], -1);
                }
            } else {
                if !arr[2] {
                    step_backward = true;
                    continue;
                } else {
                    stack.push(3);
                    x += 1;
                    modify_hash_map(&mut all_slices, [y, z], -1);
                }
            }
        }
        if success {
            output_rev(stack, start, lowest, path_to_file);
            found_solution = true;
            break;
        } else {
            modify_hash_map(&mut all_slices, start, lowest); //t delet
        }
    }
    let status: &str = if found_solution { "success" } else { "failure" };
    println!("status: {}", status);
}

fn read_input(path: &str) -> (i32, HashSet<i32>, HashMap<[i32; 2], i32>, i64) {
    let file: File = File::open(path).unwrap();
    let mut all_slices: HashMap<[i32; 2], i32> = HashMap::<[i32; 2], i32>::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut lowest: i32 = n;
    let mut lowest_second: HashSet<i32> = HashSet::<i32>::new();
    for line in lines {
        let line: String = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let n1: i32 = i32::from_str(line2.next().unwrap()).unwrap();
        let n2: i32 = i32::from_str(line2.next().unwrap()).unwrap();
        if n1 < lowest {
            lowest = n1;
            lowest_second.clear();
        }
        if n1 == lowest {
            lowest_second.insert(n2);
        }
        let nums: [i32; 2] = [n1, n2];
        *all_slices.entry(nums).or_insert(0) += 1;
    }
    (lowest, lowest_second, all_slices, n as i64)
}

fn modify_hash_map(map: &mut HashMap<[i32; 2], i32>, key: [i32; 2], val: i32) {
    // modify and auto remove keys with value zero
    *map.entry(key).or_insert(0) += val;
    if map[&key] == 0 {
        map.remove(&key);
    }
}

fn back_backtrack(state: i32, x: &mut i32, y: &mut i32, z: &mut i32) -> [i32; 2] {
    // maps state and updates dimensions
    if state == 1 {
        *z -= 1;
        [*x, *y]
    } else if state == 2 {
        *y -= 1;
        [*x, *z]
    } else {
        *x -= 1;
        [*y, *z]
    }
}

fn output_rev(stack: Vec<i32>, start: [i32; 2], lowest: i32, file_name: &str) {
    // reconstruct slices from stack
    let mut x: i32 = lowest;
    let mut y: i32 = start[0];
    let mut z: i32 = start[1];
    let mut output: String = String::new();
    // start slices
    for _ in 0..lowest {
        output.push_str(&format!("{}, {}\n", y, z));
    }
    // slices on the stack
    for state in stack {
        order(&mut x, &mut y, &mut z);
        if state == 1 {
            z += 1;
            output.push_str(&format!("{}, {}\n", x, y));
        } else if state == 2 {
            y += 1;
            output.push_str(&format!("{}, {}\n", x, z));
        } else {
            // state == 3
            x += 1;
            output.push_str(&format!("{}, {}\n", y, z));
        }
    }
    // output slices
    let path: String;
    if RUN_WITH_CARGO {
        path = format!("../output/{}", file_name);
    } else {
        path = format!("output/{}", file_name);
    }
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn order(x: &mut i32, y: &mut i32, z: &mut i32) {
    // order x, y, z in ascending order
    if *x <= *y && *x <= *z {
        if *z <= *y {
            (*z, *y) = (*y, *z);
        }
    } else if *y <= *z && *y <= *x {
        if *x <= *z {
            (*x, *y) = (*y, *x);
        } else {
            (*x, *y, *z) = (*y, *z, *x);
        }
    } else {
        if *x <= *y {
            (*x, *y, *z) = (*z, *x, *y);
        } else {
            (*x, *z) = (*z, *x);
        }
    }
}
