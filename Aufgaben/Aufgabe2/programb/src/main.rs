use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::stdin;
use std::io::{Write, BufRead, BufReader};
use std::str::FromStr;
use std::time::{Duration, Instant};


const DISPLAY_MODE:bool = false;
const PRINT_SOLUTION:bool = false;
const PRINT_TIME_ALL:bool = true;
const PRINT_TIME_TOTAL:bool = true;
const WAIT_FOR_END_ENTER:bool = false;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let start_time_all:Instant = Instant::now(); // PRINT_TIME
    for i in 1..8 {
        let start_time:Instant = Instant::now(); // PRINT_TIME
        if PRINT_TIME_ALL {
            println!("Test: {}", i);
        }
        let (lowest, lowest_second, all_slices, n) = read_input(i);
        if DISPLAY_MODE {println!("\nTest {}", i)}
        solve(lowest, lowest_second, all_slices, i, n);
        if PRINT_TIME_ALL {
            let timedelta:Duration = start_time.elapsed();
            let time:f32 = timedelta.as_secs_f32();
            println!("Gesammte Zeit für den Test in Sekunden: {}\n", time);
        }
    }
    if PRINT_TIME_TOTAL {
        let timedelta:Duration = start_time_all.elapsed();
        let time:f32 = timedelta.as_secs_f32();
        println!("Zeit für alle Tests in Sekunden: {}", time);
    }
    if WAIT_FOR_END_ENTER {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    }
}


fn order(x:&mut i32, y:&mut i32, z:&mut i32) {
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


fn solve(lowest:i32, lowest_second:HashSet<i32>, all_slices:HashMap::<[i32;2], i32>, test_number:i32, n:i64) {
    let mut start_slices: Vec<[i32;2]> = Vec::new();   
    let start_time:Instant = Instant::now(); // PRINT_TIME_ALL
    for (key, value) in all_slices.iter() {
        if *value >= lowest && (lowest_second.contains(&key[0]) || lowest_second.contains(&key[1])){
            start_slices.push(key.clone());
        }
    }
    if DISPLAY_MODE {
        println!("\n{:?}", lowest);
        println!("{:?}", start_slices);
    }
    let mut found_solution:bool = false;
    while !start_slices.is_empty() {
        let start: [i32;2] = start_slices.pop().unwrap();
        let mut x:i32 = start[0];
        let mut y:i32 = start[1];
        let mut current_map:HashMap::<[i32;2], i32> = all_slices.clone();
        let mut z:i32 = lowest;
        modify_hash_map(&mut current_map, start, -lowest);
        let mut stack: Vec<i32> = Vec::new();
        let mut step_backward:bool = false;
        order(&mut x, &mut y, &mut z);
        let mut state:i32;
        let mut success:bool = true;
        while !current_map.is_empty() || step_backward {
            if DISPLAY_MODE {println!("{},{},{} {:?}", x,y,z,current_map);}
            if step_backward {
                if DISPLAY_MODE {println!("in step_backwards");}
                order(&mut x, &mut y, &mut z);
                if stack.is_empty() {
                    success = false;
                    break;
                }
                state = stack.pop().unwrap();
                let slice = back_backtrack(state, &mut x, &mut y, &mut z);
                modify_hash_map(&mut current_map, slice, 1);
                if state == 3 {
                    continue;
                } else {
                    step_backward = false;
                }
            } else {
                state = 0;
            }
            order(&mut x, &mut y, &mut z);
            let arr:[bool;3] = [
                current_map.contains_key(&[x, y]),
                current_map.contains_key(&[x, z]),
                current_map.contains_key(&[y, z]),
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
                            modify_hash_map(&mut current_map, [y,z], -1);
                        }
                    } else {
                        stack.push(2);
                        y += 1;
                        modify_hash_map(&mut current_map, [x,z], -1);
                    }
                } else {
                    stack.push(1);
                    z += 1;
                    modify_hash_map(&mut current_map, [x,y], -1);
                }
            } else if state == 1 {
                if !arr[1] {
                    if !arr[2] {
                        step_backward = true;
                        continue;
                    } else {
                        stack.push(3);
                        x += 1;
                        modify_hash_map(&mut current_map, [y,z], -1);
                    }
                } else {
                    stack.push(2);
                    y += 1;
                    modify_hash_map(&mut current_map, [x,z], -1);
                }
            } else {
                if !arr[2] {
                    step_backward = true;
                    continue;
                } else {
                    stack.push(3);
                    x += 1;
                    modify_hash_map(&mut current_map, [y,z], -1);
                } 
            }
        }
        if DISPLAY_MODE {println!("ende");}
        if success {
            if PRINT_SOLUTION {println!("funktioniert");}
            if PRINT_TIME_ALL {
                let timedelta:Duration = start_time.elapsed();
                let factor:i64 = (timedelta.as_nanos() as i64) / n;
                let time:f32 = timedelta.as_secs_f32();
                println!("success");
                println!("Zeit in Sekunden ohne Eingabe/Ausgabe lesen/schreiben: {}", time);
                println!("Nanosekunden pro Käsescheibe: {}", factor);
            }
            output_rev(stack, start, lowest, test_number);
            found_solution = true;
            break;
        } else {
            if PRINT_SOLUTION {println!("funktioniert nicht");}
        }
    }
    if PRINT_TIME_ALL && !found_solution {
        let timedelta:Duration = start_time.elapsed();
        let factor:i64 = (timedelta.as_nanos() as i64) / n;
        let time:f32 = timedelta.as_secs_f32();
        println!("failure");
        println!("Zeit in Sekunden ohne Eingabe/Ausgabe lesen/schreiben: {}", time);
        println!("Nanosekunden pro Käsescheibe: {}", factor);
    }
} 


fn read_input(number_of_test: i32) -> (i32, HashSet<i32>, HashMap::<[i32; 2], i32>, i64) {
    let s:String = format!("../testcases/bsp{}.txt", number_of_test.to_string());
    let file:File = File::open(s).unwrap();
    let mut all_slices:HashMap<[i32; 2], i32> = HashMap::<[i32; 2], i32>::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut lowest:i32 = n;
    let mut lowest_second:HashSet<i32> = HashSet::<i32>::new();
    for line in lines {
        let line:String = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let n1:i32 = i32::from_str(line2.next().unwrap()).unwrap();
        let n2:i32 = i32::from_str(line2.next().unwrap()).unwrap();
        if n1 < lowest {
            lowest = n1;
            lowest_second.clear();
        }
        if n1 == lowest {
            lowest_second.insert(n2);
        }
        let nums: [i32; 2] = [n1, n2];
        *all_slices.entry(nums).or_insert(0) += 1;
    };
    (lowest, lowest_second, all_slices, n as i64)
}


fn modify_hash_map(map: &mut HashMap<[i32; 2], i32>, key: [i32; 2], val: i32) {
    *map.entry(key).or_insert(0) += val;
    if map[&key] == 0 {
        map.remove(&key);
    }
}


fn back_backtrack(state: i32, x:&mut i32, y:&mut i32, z:&mut i32) -> [i32;2]{
    // maps state and updates dimensions 
    // could surely be optimized
    if state == 1 {
        *z -=1;
        [*x,*y]
    } else if state == 2 {
        *y -=1;
        [*x,*z]
    } else {
        *x -=1;
        [*y,*z]
    }
}


fn output_rev(stack:Vec<i32>, start:[i32;2], lowest:i32, test_number:i32) {
    let mut x:i32 = lowest;
    let mut y:i32 = start[0];
    let mut z:i32 = start[1];
    let mut output:String = String::new();
    for _ in 0..lowest {
        output.push_str(&format!("{}, {}\n", y, z));
    }
    for state in stack {
        order(&mut x, &mut y, &mut z);
        if state == 1 {
            z +=1;
            output.push_str(&format!("{}, {}\n", x, y));
        } else if state == 2 {
            y +=1;
            output.push_str(&format!("{}, {}\n", x, z));
        } else { // state == 3
            x +=1;
            output.push_str(&format!("{}, {}\n", y, z));
        }
    }
    if PRINT_SOLUTION {
        println!("{}", output);
    }
    let path:String = format!("../output/test{}.txt", test_number.to_string());
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
