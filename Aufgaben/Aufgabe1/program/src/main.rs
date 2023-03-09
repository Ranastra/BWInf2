// this is the completely greedy approach for now
use std::f32::consts::PI;
use std::f32;
use std::str::FromStr;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::collections::HashSet;
use std::cmp;
use rand::seq::SliceRandom;
use std::collections::VecDeque;


const HALF_PI:f32 = PI/2.0;

const PRINT0:bool = true; // steps 
const PRINT1:bool = false; // stack modified
const PRINT2:bool = true; // just result
const PRINT3:bool = false; // lenght oof stack and added or removed point
const RAND_MODE:bool = true; // whether or not the algorithm should be fixed by randomnes

const MAX_ACTION_COUNT:i32 = 10_000; // limit for rand mode

fn main() {
    if false {
        // mini test
        let f1:f32 = 0.0;
        let f2:f32 = 3.0;
        let f3:f32 = 4.0;
        let n:i32 = 3;
        let points = vec!([f1, f1], [f3, f1], [f1, f2]);
        let all_distances = get_all_distances(n, &points);
        let all_angles = get_all_angles(n, &all_distances);
        let (td, stack) = solve_greedy1(n, all_distances, all_angles);
        terminal_output(td, stack, points);
    } else {
        for test in 1..8 {
            //if test == 5 {continue;} // muss debugt werden
            let (n, points) = read_input(test);
            if PRINT0 {println!("after reading input");}
            let all_distances:Vec<Vec<f32>> = get_all_distances(n, &points);
            if PRINT0 {println!("after distances");}
            let all_angles:Vec<Vec<Vec<bool>>> = get_all_angles(n, &all_distances);
            if PRINT0 {println!("after angles");}
            let (td, stack) = solve_greedy1(n, all_distances, all_angles);
            if PRINT0 {println!("after solve");}
            // terminal_output(td, stack, points);
            output(td, stack, points, test);
            if PRINT0 {println!("after output");}
            if PRINT2 {println!("finished {}", test);}
        }
    }
}

fn distance(x0:f32, y0:f32, x1:f32, y1:f32) -> f32 {
    let difx:f32 = x0-x1;
    let dify:f32 = y0-y1;
    (difx*difx + dify*dify).sqrt()
}

fn angle(d1:f32, d2:f32, d3:f32) -> bool {
    let cosa:f32 = (d1*d1 + d2*d2 - d3*d3)/(2.0*d1*d2);
    cosa.acos() >= HALF_PI
}


fn read_input(number_of_test: i32) -> (i32, Vec::<[f32; 2]>) {
    let s:String = format!("../testcases/bsp{}.txt", number_of_test.to_string());
    let file:File = File::open(s).unwrap();
    let mut all_points:Vec<[f32; 2]> = Vec::<[f32; 2]>::new();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut n: i32 = 0;
    for line in lines {
        let line:String = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        let x:f32 = f32::from_str(line2.next().unwrap()).unwrap();
        let y:f32 = f32::from_str(line2.next().unwrap()).unwrap();
        let point: [f32; 2] = [x, y];
        all_points.push(point);
        n +=1;
    };
    (n, all_points)
}

fn get_all_distances(n:i32, ap:&Vec::<[f32;2]>) -> Vec<Vec<f32>> {
    let n:usize = n as usize;
    let mut all_distances:Vec<Vec<f32>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            if j == i {
                break;
            } else {
                let dist = distance(ap[i][0], ap[i][1], ap[j][0], ap[j][1]);
                all_distances[i][j] = dist;
                all_distances[j][i] = dist;
            }
        }
    }
    all_distances
}

fn get_all_angles(n:i32, ad:&Vec<Vec<f32>>) -> Vec<Vec<Vec<bool>>>{
    // realy high memory usage .... 18kb
    let n:usize = n as usize;
    let mut all_angles:Vec<Vec<Vec<bool>>> = vec![vec![vec![false; n]; n]; n];
    for i in 0..n {
        for j in (0..n).rev() {
            if i == j {
                break;
            } else {
                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    } else {
                        let angle = angle(ad[k][j], ad[k][i], ad[i][j]);
                        all_angles[i][k][j] = angle;
                        all_angles[j][k][i] = angle;
                    }
                }
            }
        }
    }
    all_angles
}

fn solve_greedy1(n:i32, ad:Vec<Vec<f32>>, aa:Vec<Vec<Vec<bool>>>) -> (f32, Vec<usize>){
    let n:usize = n as usize;
    let mut stack:Vec<usize> = Vec::new();
    let mut visited:HashSet<usize> = HashSet::new();
    let mut found_path:bool = false;
    let mut start_points:Vec<usize> = (0..n).collect();
    if RAND_MODE {start_points.shuffle(&mut rand::thread_rng());}
    let mut action_count:i32 = 0;
    'pathfinder: for sp1 in start_points {
        stack.push(sp1);
        if PRINT1 {println!("{:?}", stack);}
        if PRINT3 {println!("{} {}", stack.len(), sp1);}
        visited.insert(sp1);
        let ordered_distances:Vec<usize> = get_ordered_distances(&ad[sp1]);
        for sp2 in ordered_distances {
            if sp1 == sp2 {continue;}
            stack.push(sp2);
            if PRINT1 {println!("{:?}", stack);}
            if PRINT3 {println!("{} {}", stack.len(), sp2);}
            visited.insert(sp2);
            let mut backwards:bool = false;
            let mut last_on_place:usize = sp1;
            // Backtracking
            loop {
                if backwards {
                    if stack.len() == 2 {
                        // kein path gefunden für diesen 2. Knoten
                        break;
                    } else {
                        last_on_place = stack.pop().unwrap();
                        if PRINT1 {println!("{:?}", stack);}
                        if PRINT3 {println!("{} {}", stack.len(), last_on_place);}
                        visited.remove(&last_on_place);
                        backwards = false;
                    }
                } else {
                    // find next point
                    let last_distance:f32 = {if last_on_place == sp1 {0.0} else {ad[*(stack.last().unwrap())][last_on_place]}};
                    let mut best:usize = n;
                    let last_num: usize = stack[stack.len()-1];
                    let last_last_num: usize = stack[stack.len()-2];
                    for next_point in 0..n {
                        if visited.contains(&next_point) {
                            // skip when point is already visited
                            continue;
                        } else if !aa[last_last_num][last_num][next_point] {
                            // skip if the angle is too small
                            continue;
                        } else if last_distance > ad[last_num][next_point] {
                            // skip cause distance of last choosen before backwards is longer
                            continue;
                        } else if last_distance == ad[last_num][next_point] && last_on_place >= next_point {
                            // skip because this next_point was already checked
                            continue;
                        } else if  best != n && ad[best][last_num] < ad[next_point][last_num] {
                            // skip cause this is worse than the best distance
                            continue;
                        } else if best != n && ad[best][last_num] == ad[next_point][last_num] && next_point > best {
                            // if distances are the same the point with lower index should be choosen
                            continue;
                        } else {
                            // better next point was found
                            best = next_point;
                        }
                    }
                    // apply next point
                    if best == n {
                        // no new point so backward
                        backwards = true;
                        if PRINT1 {println!("backstep");}
                    } else {
                        last_on_place = sp1;
                        stack.push(best);
                        if PRINT1 {println!("{:?}", stack);}
                        if PRINT3 {println!("{} {}", stack.len(), best);}
                        visited.insert(best);
                        if RAND_MODE {
                            action_count += 1;
                            if action_count == MAX_ACTION_COUNT {
                                // just restart for a new first point
                                visited.clear();
                                stack.clear();
                                action_count = 0;
                                continue 'pathfinder;
                            }
                        }
                    }
                    if stack.len() == n {
                        found_path = true;
                        if PRINT0 {println!("found solution");}
                        break 'pathfinder;
                    }
                }
            }
            visited.remove(&sp2);
            stack.pop();
            if PRINT1 {println!("{:?}", stack);}
            if PRINT3 {println!("{} {}", stack.len(), sp2);}
        }
        visited.remove(&sp1);
        stack.pop();
        if PRINT1 {println!("{:?}", stack);}
        if PRINT3 {println!("{} {}", stack.len(), sp1);}
    }
    //output(stack, current_path_length);
    //println!("{}, {:?}", current_path_length, stack);
    if found_path {

    }
    let current_path_length = {if found_path {calculate_path_length(&stack, &ad)} else {0.0}};
    (current_path_length, stack)
}

fn solve_greedy2(n:i32, ad:Vec<Vec<f32>>, aa:Vec<Vec<Vec<bool>>>) -> (f32, Vec<usize>){
    let n:usize = n as usize;
    let mut deque:VecDeque<usize> = VecDeque::new();
    let mut stack_mode:Vec<bool> = Vec::new();
    let mut visited:HashSet<usize> = HashSet::new();
    let mut found_path:bool = false;
    let mut start_points:Vec<usize> = (0..n).collect();
    if RAND_MODE {start_points.shuffle(&mut rand::thread_rng());}
    let mut action_count:i32 = 0;
    'pathfinder: for sp1 in start_points {
        deque.push_back(sp1);
        if PRINT1 {println!("{:?}", deque);}
        if PRINT3 {println!("{} {}", deque.len(), sp1);}
        visited.insert(sp1);
        let ordered_distances:Vec<usize> = get_ordered_distances(&ad[sp1]);
        for sp2 in ordered_distances {
            if sp1 == sp2 {continue;}
            deque.push_back(sp2);
            if PRINT1 {println!("{:?}", deque);}
            if PRINT3 {println!("{} {}", deque.len(), sp2);}
            visited.insert(sp2);
            let mut backwards:bool = false;
            let mut last_on_place_right:usize = n;
            let mut last_on_place_left:usize = n;
            // Backtracking
            loop {
                if backwards {
                    if deque.len() == 2 {
                        // kein path gefunden für diesen 2. Knoten
                        break;
                    } else {
                        let mode:bool = stack_mode.pop().unwrap();
                        if mode {
                            last_on_place_right = deque.pop_back().unwrap();
                            visited.remove(&last_on_place_right);
                        } else {
                            last_on_place_left = deque.pop_front().unwrap();
                            visited.remove(&last_on_place_left);
                        }
                        if PRINT1 {println!("{:?}", deque);}
                        if PRINT3 {println!("{} ({} {})", deque.len(), last_on_place_left, last_on_place_right);}
                        backwards = false;
                    }
                } else {
                    // find next point
                    let last_distance_right:f32 = {if last_on_place_right == n {0.0} else {ad[*(deque.back().unwrap())][last_on_place_right]}};
                    let last_distance_left:f32 = {if last_on_place_left == n {0.0} else {ad[deque[0]][last_on_place_left]}};
                    let mut best:usize = n;
                    let mut best_distance:f32 = 0.0;
                    let last_num:usize = deque[deque.len()-1];
                    let last_last_num:usize = deque[deque.len()-2];
                    let first_num:usize = deque[0];
                    let second_num:usize = deque[1];
                    let mut best_mode:bool = true;
                    for next_point in 0..n {
                        // if visited.contains(&next_point) {
                        //     // skip when point is already visited
                        //     continue;
                        // } else if !aa[last_last_num][last_num][next_point] {
                        //     // skip if the angle is too small
                        //     continue;
                        // } else if last_distance > ad[last_num][next_point] {
                        //     // skip cause distance of last choosen before backwards is longer
                        //     continue;
                        // } else if last_distance == ad[last_num][next_point] && last_on_place >= next_point {
                        //     // skip because this next_point was already checked
                        //     continue;
                        // } else if  best != n && ad[best][last_num] < ad[next_point][last_num] {
                        //     // skip cause this is worse than the best distance
                        //     continue;
                        // } else if best != n && ad[best][last_num] == ad[next_point][last_num] && next_point > best {
                        //     // if distances are the same the point with lower index should be choosen
                        //     continue;
                        // } else {
                        //     // better next point was found
                        //     best = next_point;
                        // }
                        if visited.contains(&next_point) {
                            // skip when point is already visited
                        } else {
                            if aa[last_last_num][last_last_num][next_point] && 
                               !(last_distance_right > ad[last_num][next_point]) && 
                               !(last_distance_right == ad[last_num][next_point] && last_on_place_right >= next_point) &&
                               !(best != n && ad[best][last_num] < ad[next_point][last_num]) &&
                               !(best != n && ad[best][last_num] == ad[next_point][last_num] && next_point > best) {
                                
                            }
                            if aa[second_num][first_num][next_point] {

                            }
                        }
                    }
                    // apply next point
                    if best == n {
                        // no new point so backward
                        backwards = true;
                        if PRINT1 {println!("backstep");}
                    } else {
                        last_on_place = sp1;
                        stack.push(best);
                        if PRINT1 {println!("{:?}", deque);}
                        if PRINT3 {println!("{} {}", deque.len(), best);}
                        visited.insert(best);
                        if RAND_MODE {
                            action_count += 1;
                            if action_count == MAX_ACTION_COUNT {
                                // just restart for a new first point
                                visited.clear();
                                deque.clear();
                                action_count = 0;
                                continue 'pathfinder;
                            }
                        }
                    }
                    if deque.len() == n {
                        found_path = true;
                        if PRINT0 {println!("found solution");}
                        break 'pathfinder;
                    }
                }
            }
            visited.remove(&sp2);
            deque.pop_back();
            if PRINT1 {println!("{:?}", deque);}
            if PRINT3 {println!("{} {}", deque.len(), sp2);}
        }
        visited.remove(&sp1);
        deque.pop_back();
        if PRINT1 {println!("{:?}", deque);}
        if PRINT3 {println!("{} {}", deque.len(), sp1);}
    }
    //output(stack, current_path_length);
    //println!("{}, {:?}", current_path_length, stack);
    if found_path {

    }
    let current_path_length = {if found_path {calculate_path_length(&deque, &ad)} else {0.0}};
    (current_path_length, deque)
}


fn terminal_output(td:f32, stack:Vec<usize>, points:Vec<[f32;2]>) {
    println!("Distanz: {}", td);
    for point in stack {
        println!("{:?}, ", points[point as usize]);
    }
    println!();
}

fn order_distances(tup1:&(f32, usize), tup2:&(f32, usize)) -> cmp::Ordering {
    tup1.0.partial_cmp(&tup2.0).unwrap()
}

fn get_ordered_distances(distances:&Vec<f32>) -> Vec<usize> {
    let mut ordered_distances:Vec<(f32, usize)> = Vec::new();
    for (ind, distance) in distances.iter().enumerate() {
        ordered_distances.push((*distance, ind));
    }
    ordered_distances.sort_by(order_distances);
    let mut indexes:Vec<usize> = Vec::new();
    for (_distance, ind) in ordered_distances {
        indexes.push(ind);
    }
    indexes
}

fn calculate_path_length(stack:&Vec<usize>, ad:&Vec<Vec<f32>>) -> f32 {
    // calculates the total distance given the path and the all_distances map
    let mut total_distance:f32 = 0.0;
    for i in 1..stack.len() as usize {
        total_distance += ad[i-1][i];
    }
    total_distance
}

fn output(total_distance:f32, stack:Vec<usize>, points:Vec<[f32;2]>, test_number:i32) {
    let mut output:String = String::from(format!("{}\n", total_distance));
    for ind in stack {
        let [x, y] = points[ind];
        output.push_str(&format!("{}, {}\n", x, y));
    }
    let path: String = format!("../output/test{}.txt", test_number);
    let mut file = File::create(path).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}