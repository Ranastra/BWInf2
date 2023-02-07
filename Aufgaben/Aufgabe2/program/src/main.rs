use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::str::FromStr;
use std::env;

const DISPLAY_MODE:bool = false;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    for i in 7..8 {
        let (lowest, all_slices) = read_input(i);
        if DISPLAY_MODE {println!("\nTest {}", i)}
        solve(lowest, all_slices);
    }
    // let mut x = 3;
    // let mut y = 2;
    // let mut z = 1;
    // order(&mut x, &mut y, &mut z);
    // println!("{}, {}, {}", x, y, z);
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


fn solve(lowest:[i32;2], all_slices:HashMap::<[i32;2], i32>) {
    // println!("{}", n);
    // println!("{}, {}", lowest[0], lowest[1]);
    let mut start_slices: Vec<[i32;2]> = Vec::new();
    for (key, value) in all_slices.iter() {
        if *value>=lowest[0] && (lowest[1]==key[0] || lowest[1]==key[1]){
            // println!("{} {}, {}", key[0], key[1], value);
            start_slices.push(key.clone());
        }
    }
    if DISPLAY_MODE {println!("{:?}", start_slices);}
    while !start_slices.is_empty() {
        let start: [i32;2] = start_slices.pop().unwrap();
        //println!("{:?}", start);
        let mut x:i32 = start[0];
        let mut y:i32 = start[1];
        let mut current_map:HashMap::<[i32;2], i32> = all_slices.clone();
        let mut z:i32 = current_map.remove(&start).unwrap();
        let mut stack: Vec<i32> = Vec::new();
        let mut step_backward = false;
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
            if DISPLAY_MODE {println!("success");}
            println!("test");
            break;
        }
    }
} 


fn read_input(number_of_test: i32) -> ([i32; 2], HashMap::<[i32; 2], i32>) {
    let s = format!("../testcases/bsp{}.txt", number_of_test.to_string());
    let file = File::open(s).unwrap();
    let mut all_slices = HashMap::<[i32; 2], i32>::new();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let n: i32 = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    let mut lowest = [n,n];
    for line in lines {
        let line = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let n1 = i32::from_str(line2.next().unwrap()).unwrap();
        let n2 = i32::from_str(line2.next().unwrap()).unwrap();
        if n1 < lowest[0] || (n1 == lowest[0] && n2 < lowest[1]) {
            lowest = [n1, n2];
        }
        let nums: [i32; 2] = [n1, n2];
        *all_slices.entry(nums).or_insert(0) += 1;
    };
    (lowest, all_slices)
}


fn modify_hash_map(map: &mut HashMap<[i32; 2], i32>, key: [i32; 2], val: i32) {
    // let new_count = map.remove(&key).unwrap() + val;
    // if new_count != 0 {
    //     map.insert(key, new_count);
    // }
    // ChatGPT meint das wär schneller
    *map.entry(key).or_insert(0) += val;
    if map[&key] == 0 {
        map.remove(&key);
    }
}


fn back_backtrack(state: i32, x:&mut i32, y:&mut i32, z:&mut i32) -> [i32;2]{
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
