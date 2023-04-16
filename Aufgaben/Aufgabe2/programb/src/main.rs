use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use std::time::{Duration, Instant};

const DISPLAY_MODE: bool = false;
const PRINT_TIME_ALL: bool = true;
const PRINT_TIME_TOTAL: bool = true;
const RUN_WITH_CARGO: bool = false;
const PRINT_REMOVED: bool = true;
const NUMBER_EATEN: usize = 2;

fn main() {
    if RUN_WITH_CARGO {
        run_all(NUMBER_EATEN);
    } else {
        // get arguments from command line filename
        let args: Vec<String> = env::args().collect();
        let filename = &args[1];
        let number_eaten: usize = args[2].parse().unwrap();
        if filename == "all" {
            run_all(number_eaten);
        } else {
            run_one(filename, number_eaten);
        }
    }
}

fn run_one(filename: &str, number_eaten: usize) {
    println!("\nTest {}", filename);
    let start_time: Instant = Instant::now();
    let path_to_file: String;
    // format path to file
    if RUN_WITH_CARGO {
        path_to_file = format!("../testcasesb/{}", filename);
    } else {
        path_to_file = format!("testcasesb/{}", filename);
    }
    // read input
    let (lowest, lowest_second, all_slices, _n) = read_input(&path_to_file);
    // solve
    solve(
        lowest as usize,
        lowest_second,
        all_slices,
        filename,
        number_eaten,
    );
    if PRINT_TIME_ALL {
        let duration: Duration = start_time.elapsed();
        println!("time for one testcase: {:?}", duration);
    }
}

fn run_all(number_eaten: usize) {
    // run all example testcases
    let start_time: Instant = Instant::now(); // PRINT_TIME_ALL
    for i in 1..8 {
        let filename: String = format!("bsp{}.txt", i);
        run_one(&filename, number_eaten);
    }
    if PRINT_TIME_TOTAL {
        let duration: Duration = start_time.elapsed();
        println!("Total time for all Testcases: {:?}", duration);
    }
}

fn solve(
    lowest: usize,
    lowest_second: HashSet<usize>,
    mut all_slices: HashMap<[usize; 2], usize>,
    file_name: &str,
    number_eaten: usize,
) {
    let number_eaten: usize = number_eaten;
    let mut start_slices: Vec<[usize; 2]> = Vec::new(); // change all_slices to mut
    if DISPLAY_MODE {
        println!("starting test\nall_slices: {:?}", all_slices);
        println!("lowest: {:?}, lowest_second: {:?}", lowest, lowest_second);
    }
    for (key, value) in all_slices.iter() {
        if *value >= lowest && (lowest_second.contains(&key[0]) || lowest_second.contains(&key[1]))
        {
            start_slices.push(key.clone());
        }
    }
    if DISPLAY_MODE {
        println!("\nlowest: {:?}", lowest);
        println!("start_slices: {:?}", start_slices);
    }
    while !start_slices.is_empty() {
        let start: [usize; 2] = start_slices.pop().unwrap();
        let mut x: usize = start[0];
        let mut y: usize = start[1];
        let mut z: usize = lowest;
        modify_hash_map(&mut all_slices, start, -(lowest as i32)); //t
        let mut stack: Vec<([bool; 3], usize, bool)> = Vec::new();
        let mut step_backward: bool = false;
        order(&mut x, &mut y, &mut z);
        let mut state: ([bool; 3], usize, bool);
        let mut success: bool = true;
        let mut made_up_slices: HashMap<[usize; 2], usize> = HashMap::new();
        let mut made_up_slices_count: usize = 0;
        if DISPLAY_MODE {
            println!("||| start_slice:  {:?}", start);
        }
        while !all_slices.is_empty() || step_backward {
            if step_backward {
                if DISPLAY_MODE {
                    println!("in step_backwards");
                }
                order(&mut x, &mut y, &mut z);
                if stack.is_empty() {
                    success = false;
                    break;
                }
                state = stack.pop().unwrap();
                if DISPLAY_MODE {
                    println!("\nin backtrack");
                    //println!("stack: {:?}", stack);
                    println!("made_up_slices: {:?}", made_up_slices);
                    println!("all_slices: {:?}", all_slices);
                }
                let slice = back_backtrack(state.1, &mut x, &mut y, &mut z);
                if DISPLAY_MODE {
                    println!("state from stack: {:?}", state);
                    println!("slice from state: {:?}\n", slice);
                }
                if !state.2 {
                    if DISPLAY_MODE {
                        print!("all_slices ");
                    }
                    modify_hash_map(&mut all_slices, slice, 1); //t
                } else {
                    if DISPLAY_MODE {
                        print!("made_up_slices ");
                    }
                    modify_hash_map(&mut made_up_slices, slice, -1);
                    made_up_slices_count -= 1;
                }
                if DISPLAY_MODE {
                    if DISPLAY_MODE {
                        print!("made_up_slices ");
                    }
                    println!("made_up_slices: {:?}", made_up_slices);
                }
                if state.0 == [true, true, true] {
                    continue;
                } else {
                    step_backward = false;
                }
            } else {
                let empty: [bool; 3] = [false, false, false];
                state = (empty, 0, false);
            }
            order(&mut x, &mut y, &mut z);
            let arr: [bool; 3] = [
                all_slices.contains_key(&[x, y]),
                all_slices.contains_key(&[x, z]),
                all_slices.contains_key(&[y, z]),
            ];
            let mut found_slice: bool = false;
            if !state.2 {
                if DISPLAY_MODE {
                    println!("in normal");
                }
                if !state.0[0] && arr[0] {
                    state.0[0] = true;
                    state.1 = 1;
                    stack.push(state);
                    z += 1;
                    found_slice = true;
                    if DISPLAY_MODE {
                        print!("all_slices ");
                    }
                    modify_hash_map(&mut all_slices, [x, y], -1)
                } else if !state.0[1] && arr[1] {
                    state.0[1] = true;
                    state.1 = 2;
                    y += 1;
                    if y > z {
                        state.1 = 1;
                    }
                    stack.push(state);
                    found_slice = true;
                    if DISPLAY_MODE {
                        print!("all_slices ");
                    }
                    modify_hash_map(&mut all_slices, [x, z], -1);
                } else if !state.0[2] && arr[2] {
                    state.0[2] = true;
                    state.1 = 3;
                    x += 1;
                    if x > y {
                        state.1 = 2;
                        if x > z {
                            state.1 = 1;
                        }
                    }
                    stack.push(state);
                    found_slice = true;
                    if DISPLAY_MODE {
                        print!("all_slices ");
                    }
                    modify_hash_map(&mut all_slices, [y, z], -1);
                } else {
                    state.2 = true;
                }
            }
            if state.2 && made_up_slices_count < number_eaten {
                if DISPLAY_MODE {
                    println!("in made_up");
                }
                made_up_slices_count += 1;
                if !state.0[0] {
                    state.0[0] = true;
                    state.1 = 1;
                    stack.push(state);
                    z += 1;
                    found_slice = true;
                    if DISPLAY_MODE {
                        println!("created slice {} {}", x, y);
                        print!("made_up_slices ")
                    }
                    modify_hash_map(&mut made_up_slices, [x, y], 1)
                } else if !state.0[1] {
                    state.0[1] = true;
                    state.1 = 2;
                    y += 1;
                    if y > z {
                        state.1 = 1;
                    }
                    stack.push(state);
                    found_slice = true;
                    if DISPLAY_MODE {
                        println!("created slice {} {}", x, z);
                        print!("made_up_slices ")
                    }
                    modify_hash_map(&mut made_up_slices, [x, z], 1);
                } else if !state.0[2] {
                    state.0[2] = true;
                    state.1 = 3;
                    x += 1;
                    if x > y {
                        state.1 = 2;
                        if x > z {
                            state.1 = 1;
                        }
                    }
                    stack.push(state);
                    found_slice = true;
                    if DISPLAY_MODE {
                        println!("created slice {} {}", y, z);
                        print!("made_up_slices ")
                    }
                    modify_hash_map(&mut made_up_slices, [y, z], 1);
                } else {
                    made_up_slices_count -= 1;
                }
            } else {
                step_backward = true;
            }
            if found_slice {
                step_backward = false;
            }
            if DISPLAY_MODE {
                //println!("\nstack: {:?}", stack);
                println!("\nstate: {:?}", state);
                println!("made_up_slices: {:?}", made_up_slices);
                //println!("number_eaten: {}", number_eaten);
                //println!("made_up_slices_count: {}", made_up_slices_count);
                println!("step_backward: {}", step_backward);
                println!("all_slices: {:?}", all_slices);
                println!("x: {}, y: {}, z: {}", x, y, z);
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
        if success {
            output_rev(stack, start, lowest, file_name);
            if PRINT_REMOVED {
                print_removed(made_up_slices);
            }
            println!("success");
            return;
        } else {
            if DISPLAY_MODE {
                print!("all_slices ");
            }
            modify_hash_map(&mut all_slices, start, lowest as i32); //t delete
        }
    }
}

fn print_removed(removed: HashMap<[usize; 2], usize>) {
    println!("removed:\n{:?}", removed);
}

fn read_input(path: &str) -> (i32, HashSet<usize>, HashMap<[usize; 2], usize>, i64) {
    println!("path: {}", path);
    let file: File = File::open(path).unwrap();
    let mut all_slices: HashMap<[usize; 2], usize> = HashMap::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut lowest: usize = n as usize;
    let mut lowest_second: HashSet<usize> = HashSet::new();
    for line in lines {
        let line: String = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let n1: usize = usize::from_str(line2.next().unwrap()).unwrap();
        let n2: usize = usize::from_str(line2.next().unwrap()).unwrap();
        if n1 < lowest {
            lowest = n1;
            lowest_second.clear();
        }
        if n1 == lowest {
            lowest_second.insert(n2);
        }
        let nums: [usize; 2] = [n1, n2];
        *all_slices.entry(nums).or_insert(0) += 1;
    }
    (lowest as i32, lowest_second, all_slices, n as i64)
}

fn modify_hash_map(map: &mut HashMap<[usize; 2], usize>, key: [usize; 2], val: i32) {
    if DISPLAY_MODE {
        println!("modifying key: {:?}, val: {}", key, val);
        println!("map: {:?}", map);
    }
    if val < 0 {
        *map.entry(key).or_insert(0) -= (-val) as usize;
    } else {
        *map.entry(key).or_insert(0) += val as usize;
    }
    if map[&key] == 0 {
        map.remove(&key);
    }
    if DISPLAY_MODE {
        println!("map: {:?}", map);
    }
}

fn back_backtrack(state: usize, x: &mut usize, y: &mut usize, z: &mut usize) -> [usize; 2] {
    // maps state and updates dimensions
    // could surely be optimized
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

fn output_rev(
    stack: Vec<([bool; 3], usize, bool)>,
    start: [usize; 2],
    lowest: usize,
    file_name: &str,
) {
    let mut x: usize = lowest;
    let mut y: usize = start[0];
    let mut z: usize = start[1];
    let mut output: String = String::new();
    for _ in 0..lowest {
        output.push_str(&format!("{}, {}\n", y, z));
    }
    for state in stack {
        order(&mut x, &mut y, &mut z);
        if state.1 == 1 {
            z += 1;
            output.push_str(&format!("{}, {}\n", x, y));
        } else if state.1 == 2 {
            y += 1;
            output.push_str(&format!("{}, {}\n", x, z));
        } else {
            // state == 3
            x += 1;
            output.push_str(&format!("{}, {}\n", y, z));
        }
    }
    let path: String;
    if RUN_WITH_CARGO {
        path = format!("../outputb/{}", file_name);
    } else {
        path = format!("outputb/{}", file_name);
    }
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

fn order(x: &mut usize, y: &mut usize, z: &mut usize) {
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
