use std::f32::consts::PI;
use std::f32;
use std::str::FromStr;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::collections::HashSet;
use std::cmp;
use rand::seq::SliceRandom;
use std::collections::VecDeque;


const HALF_PI:f32 = PI/2.0; // 90°

const PRINT0:bool = false; // print all steps 
const PRINT1:bool = false; // print stack modified
const PRINT3:bool = false; // lenght of stack and added or removed point
const PRINT4:bool = false; // combo 1 and 3
const CUT_ACTION_COUNT:bool = true; // whether or not nearly-infinite loop should be cutted of
const RAND_FAST_MODE:bool = false; // whether or not the start points are choosen at random
const COMPARE:bool = true; // use and compare both solve0 and solve1

const MAX_ACTION_COUNT:i32 = 2_000; // limit for cut_action_count mode


fn main() {
    for test in 1..8 {
        if PRINT0 {println!("test: {}", test)}
        let (n, points) = read_input(test);
        if PRINT0 {println!("finish reading input");}
        let all_distances:Vec<Vec<f32>> = get_all_distances(n, &points);
        if PRINT0 {println!("finish distances");}
        let all_angles:Vec<Vec<Vec<bool>>> = get_all_angles(n, &all_distances);
        if PRINT0 {println!("finish angles");}
        if COMPARE {
            let mut valid_paths0:Vec<(f32, Vec<usize>)> = solve_greedy0(n, &all_distances, &all_angles);
            let mut valid_paths1:Vec<(f32, Vec<usize>)> = solve_greedy1(n, &all_distances, &all_angles);
            if PRINT0 {println!("finish solve")}
            valid_paths0.append(&mut valid_paths1);
            let mut best_stack:Vec<usize> = valid_paths0[0].clone().1;
            let mut best_distance:f32 = valid_paths0[0].0;
            let stack_len:usize = best_stack.len();
            for (td, stack) in &mut valid_paths0 {
                opt0_create_circle(td, stack, &all_distances, &all_angles);
                for i in 1..(stack_len-1) {
                    opt1_move_n_points(td, stack, &all_distances, &all_angles, i);
                }
                for i in 2..(stack_len+1) {
                    opt1_move_n_points(td, stack, &all_distances, &all_angles, stack_len-i);
                }
                if *td < best_distance {
                    best_distance = *td;
                    best_stack = stack.clone();
                }   
            }
            if PRINT0 {println!("finish optimisation")}
            output(best_distance, best_stack, points, test);
            if PRINT0 {println!("finish output")}
        } else {
            
        }
    }
}

fn distance(x0:f32, y0:f32, x1:f32, y1:f32) -> f32 {
    // calculate the distance with pythagoras
    let difx:f32 = x0-x1;
    let dify:f32 = y0-y1;
    (difx*difx + dify*dify).sqrt()
}

fn angle(d1:f32, d2:f32, d3:f32) -> bool {
    // calculate if the angle is over 90° with law of cosines
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
    // claculate the distance for al pair of points,
    // returns 2D Vec
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
    // calculate for all possible angles if they are allowed,
    // returns a 3D Vec
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

fn solve_greedy0(n:i32, ad:&Vec<Vec<f32>>, aa:&Vec<Vec<Vec<bool>>>) -> Vec<(f32, Vec<usize>)>{
    let mut valid_paths:Vec<(f32, Vec<usize>)> = Vec::new();
    let n:usize = n as usize;
    let mut stack:Vec<usize> = Vec::new();
    let mut visited:HashSet<usize> = HashSet::new();
    let mut start_points:Vec<usize> = (0..n).collect();
    if RAND_FAST_MODE {start_points.shuffle(&mut rand::thread_rng());}
    let mut action_count:i32 = 0;
    'pathfinder: for sp1 in start_points {
        stack.clear();
        stack.push(sp1);
        if PRINT1 {println!("{:?}", stack);}
        if PRINT3 {println!("{} {} sp1", stack.len(), sp1);}
        visited.insert(sp1);
        let ordered_distances:Vec<usize> = get_ordered_distances(&ad[sp1]);
        for sp2 in ordered_distances {
            if sp1 == sp2 {continue;}
            stack.push(sp2);
            if PRINT1 {println!("{:?}", stack);}
            if PRINT3 {println!("{} {} sp2", stack.len(), sp2);}
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
                        if PRINT3 {println!("{} {} backwards", stack.len(), last_on_place);}
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
                        if PRINT3 {println!("{} {} {}", stack.len(), best, last_on_place);}
                        last_on_place = sp1;
                        stack.push(best);
                        if PRINT1 {println!("{:?}", stack);}
                        visited.insert(best);
                        if CUT_ACTION_COUNT {
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
                        let tup:(f32, Vec<usize>) = (calculate_path_length(&stack, &ad), stack.clone());
                        valid_paths.push(tup);
                        if RAND_FAST_MODE {
                            return valid_paths;
                        }
                        continue 'pathfinder;
                    }
                }
            }
            visited.remove(&sp2);
            stack.pop();
            if PRINT1 {println!("{:?}", stack);}
            if PRINT3 {println!("{} {} sp2 r", stack.len(), sp2);}
        }
        visited.remove(&sp1);
        stack.pop();
        if PRINT1 {println!("{:?}", stack);}
        if PRINT3 {println!("{} {} sp1 r", stack.len(), sp1);}
    }
    valid_paths
}

fn solve_greedy1(n:i32, ad:&Vec<Vec<f32>>, aa:&Vec<Vec<Vec<bool>>>) -> Vec<(f32, Vec<usize>)>{
    let n:usize = n as usize;
    let mut deque:VecDeque<usize> = VecDeque::new();
    let mut visited:HashSet<usize> = HashSet::new();
    let mut start_points:Vec<usize> = (0..n).collect();
    let mut valid_paths:Vec<(f32, Vec<usize>)> = Vec::new();
    if RAND_FAST_MODE {start_points.shuffle(&mut rand::thread_rng());}
    let mut action_count:i32 = 0;
    'pathfinder: for sp1 in start_points {
        deque.clear();
        let mut stack_last_mode:Vec<(usize, bool)> = Vec::new();
        stack_last_mode.push((sp1, true));
        deque.push_back(sp1);
        if PRINT1 {println!("{:?}", deque);}
        if PRINT3 {println!("{} {} sp1", deque.len(), sp1);}
        visited.insert(sp1);
        let ordered_distances:Vec<usize> = get_ordered_distances(&ad[sp1]);
        for sp2 in ordered_distances {
            if sp1 == sp2 {continue;}
            stack_last_mode.push((sp2, true));
            deque.push_back(sp2);
            if PRINT1 {println!("{:?}", deque);}
            if PRINT3 {println!("{} {} sp2", deque.len(), sp2);}
            if PRINT4 {println!("{:?} Startknoten", deque);}
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
                        let back:(usize, bool) = stack_last_mode.pop().unwrap();
                        if back.1 {
                            last_on_place_right = deque.pop_back().unwrap();
                            visited.remove(&last_on_place_right);
                        } else {
                            last_on_place_left = deque.pop_front().unwrap();
                            visited.remove(&last_on_place_left);
                        }
                        if PRINT1 {println!("{:?}", deque);}
                        if PRINT3 {println!("{} ({} {}) r", deque.len(), last_on_place_left, last_on_place_right);}
                        if PRINT4 {println!("({} {}) deque {:?}", last_on_place_left, last_on_place_right, deque);}
                        backwards = false;
                    }
                } else {
                    // find next point
                    let mut best:usize = n;
                    let mut best_distance:f32 = 0.0;
                    let mut best_mode:bool = true;
                    let last_num:usize = deque[deque.len()-1];
                    let last_last_num:usize = deque[deque.len()-2];
                    let first_num:usize = deque[0];
                    let second_num:usize = deque[1];
                    for next_point in 0..n {
                        if visited.contains(&next_point) {
                            // skip when point is already visited
                        } else {
                            if aa[last_last_num][last_last_num][next_point] {
                                let distance:f32 = ad[last_num][next_point];
                                if distance < best_distance || best == n {
                                    let last_distance:f32 = {if n != last_on_place_right {ad[last_num][last_on_place_right]} else {0.0}};
                                    if (last_on_place_right == n) || 
                                       (last_distance < distance) || 
                                       (last_distance == distance && next_point > last_on_place_right) {
                                        best_mode = true;
                                        best = next_point;
                                        best_distance = distance;
                                    }
                                }
                            }
                            if aa[second_num][first_num][next_point] {
                                let distance:f32 = ad[first_num][next_point];
                                if distance < best_distance || best == n {
                                    let last_distance:f32 = {if n != last_on_place_left {ad[last_num][last_on_place_left]} else {0.0}};
                                    if (last_on_place_left == n) || 
                                       (last_distance < distance) ||
                                       (last_distance == distance && next_point > last_on_place_left) {
                                        best_mode = false;
                                        best = next_point;
                                        best_distance = distance;
                                    }
                                }
                            }
                        }
                    }
                    // apply next point
                    if PRINT4 {println!("({} {}), best: {}, deque: {:?}", last_on_place_left, last_on_place_right, best, deque);}
                    if best == n {
                        // no new point so backward
                        backwards = true;
                        if PRINT1 {println!("backstep");}
                    } else {
                        if best_mode {
                            last_on_place_right = n;
                            last_on_place_left = n;
                            deque.push_back(best);
                        } else {
                            last_on_place_left = n;
                            last_on_place_right = n;
                            deque.push_front(best);
                        }
                        if PRINT1 {println!("{:?}", deque);}
                        if PRINT3 {println!("{} {} best", deque.len(), best);}
                        visited.insert(best);
                        stack_last_mode.push((best, best_mode));
                        if CUT_ACTION_COUNT {
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
                        let tup:(f32, Vec<usize>) = (calculate_path_length(&deque.clone().into_iter().collect(), &ad), deque.clone().into_iter().collect());
                        valid_paths.push(tup);
                        if RAND_FAST_MODE {
                            return valid_paths;
                        }
                        continue 'pathfinder;
                    }
                }
            }
            visited.remove(&sp2);
            deque.pop_back();
            if PRINT1 {println!("{:?}", deque);}
            if PRINT3 {println!("{} {} sp2 r", deque.len(), sp2);}
        }
        visited.remove(&sp1);
        deque.pop_back();
        if PRINT1 {println!("{:?}", deque);}
        if PRINT3 {println!("{} {} sp1 r", deque.len(), sp1);}
    }
    valid_paths
}


fn _terminal_output(td:f32, stack:Vec<usize>, points:Vec<[f32;2]>) {
    // Prints the total distance and the index of the ordered points in the stack to the terminal.
    // for debug
    println!("Distanz: {}", td);
    for point in stack {
        println!("{:?}, ", points[point as usize]);
    }
    println!();
}

fn order_distances(tup1:&(f32, usize), tup2:&(f32, usize)) -> cmp::Ordering {
    // helper function for get_ordered_distances 
    tup1.0.partial_cmp(&tup2.0).unwrap()
}

fn get_ordered_distances(distances:&Vec<f32>) -> Vec<usize> {
    // Given a vector of distances, returns a vector of indexes
    // that correspond to the distances sorted in ascending order.
    // Create a vector of tuples, where the first element is the distance,
    // and the second element is the index.
    let mut ordered_distances:Vec<(f32, usize)> = Vec::new();
    for (ind, distance) in distances.iter().enumerate() {
        ordered_distances.push((*distance, ind));
    }
    // Sort the vector of tuples by distance.
    ordered_distances.sort_by(order_distances);
    // Extract the indexes from the sorted vector of tuples.
    let mut indexes:Vec<usize> = Vec::new();
    for (_distance, ind) in ordered_distances {
        indexes.push(ind);
    }
    indexes
}

fn calculate_path_length(stack:&Vec<usize>, ad:&Vec<Vec<f32>>) -> f32 {
    // calculates the total distance given the path and the all_distances map
    //println!("{:?}", stack); //test
    let mut total_distance:f32 = 0.0;
    for i in 1..stack.len() as usize {
        total_distance += ad[stack[i-1]][stack[i]];
        //println!("{}", ad[stack[i-1]][stack[i]]);
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

fn proove_all_angles(stack:&Vec<usize>, aa:&Vec<Vec<Vec<bool>>>) -> bool {
    // iterates through stack and checks all angles
    for i in 0..(stack.len()-2) {
        if !aa[stack[i]][stack[i+1]][stack[i+2]] {
            return false;
        }
    }
    true
}

fn opt0_create_circle(total_distance:&mut f32, stack:&mut Vec<usize>, ad:&Vec<Vec<f32>>, aa:&Vec<Vec<Vec<bool>>>) {
    // check if a circle could be created, 
    // If so, rearrange the order of points so that the start and endpoint 
    // are the ones with the longest distance in between
    let stack_len:usize = stack.len();
    // Check if last point and first point can be connected
    if aa[stack[stack_len-2]][stack[stack_len-1]][stack[0]] && aa[stack[stack_len-1]][stack[0]][stack[1]] {
        // Find the points with the longest distance in between
        let mut worst_distance:f32 = ad[stack[stack_len-1]][stack[0]];
        let mut worst_index:usize = stack_len -1;
        for i in 0..(stack_len-2) {
            if worst_distance < ad[stack[i]][stack[i+1]] {
                worst_index = i;
                worst_distance = ad[stack[i]][stack[i+1]];
            }
        }
        // If the points with the longest distance are not already the start and end points
        if worst_index != stack_len-1 {
            // create new vec with rearranged points
            let mut new_stack:Vec<usize> = Vec::new();
            for i in 0..stack_len {
                new_stack.push(stack[(i+worst_index+1)%stack_len]);
            }
            *total_distance = calculate_path_length(&new_stack, &ad);
            *stack = new_stack;
        }
    }
}

fn opt1_move_n_points(total_distance:&mut f32, stack:&mut Vec<usize>, ad:&Vec<Vec<f32>>, aa:&Vec<Vec<Vec<bool>>>, n:usize) {
    let last_distance:f32 = *total_distance; // keep track of the last total distance
    let mut new_stack:Vec<usize> = stack.clone(); // create a new stack for the new solution
    // iterate over all possible points to remove from the stack
    'outer: for choosen_point in 0..(stack.len()+1-n) { 
        // remove a slice of n points
        let mut slice:Vec<usize> = new_stack.splice(choosen_point..(choosen_point+n), std::iter::empty()).collect::<Vec<_>>();
        // iterate over all possible locations to insert the removed points
        for new_location in 0..(new_stack.len()+1) {
            // try inserting the slice in the new location, and its reverse
            for _ in 0..2 {
                // insert the removed points into the new position
                new_stack.splice(new_location..new_location, slice);
                // check if all angles are valid
                if proove_all_angles(&new_stack, aa) {
                    let distance = calculate_path_length(&new_stack, ad); //n
                    // if the new path is shorter, update the total distance and the stack
                    if distance < *total_distance {
                        *total_distance = distance;
                        *stack = new_stack.clone();
                        break 'outer;
                    }
                }
                // remove the inserted points from the stack and reverse them
                slice = new_stack.splice(new_location..(new_location+n), std::iter::empty()).collect::<Vec<_>>();
                slice.reverse();
            }
        }
        // insert the removed points back to their original position
        new_stack.splice(choosen_point..choosen_point, slice);
    }
    // if the total distance was updated, repeat the optimization
    if last_distance != *total_distance {
        opt1_move_n_points(total_distance, stack, ad, aa, n);
    }
}