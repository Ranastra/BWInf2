use std::f32::consts::PI;
use std::f32;
use std::str::FromStr;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

const HALF_PI:f32 = PI/2.0;

fn main() {
    //println!("Hello, world!");
    //println!("{}", distance(0.0, 0.0, 1.0, 1.0));
    //let mut a = new String;
    //let two:f32 = 2.0;
    //println!("{}", two.sqrt());
    //println!("{}", angle(1.0,1.0,1.4));
    //println!("{}", HALF_PI);
    if true {
        let f1:f32 = 0.0;
        let f2:f32 = 3.0;
        let f3:f32 = 4.0;
        let vecs = vec!([f1, f1], [f3, f1], [f1, f2]);
        let all_distances = get_all_distances(3, &vecs);
        let all_angles = get_all_angles(3, &all_distances);
        println!("{:?}", all_angles);
    }
    //let (n, mut test1) = read_input(1);
    //println!("{}, {:?}", n, test1);
    //let all_distances = get_all_distances(n, &mut test1);
    //let all_angles = get_all_angles(n, &all_distances);
    //println!("{:?}", all_angles);
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
    // realy high memory usage ....
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

fn solve(n:i32, ad:&Vec<Vec<f32>>, aa:&Vec<Vec<Vec<bool>>>) -> Vec<i32> {
    let mut stack:Vec<i32> = Vec::new();
    let mut visited:HashSet<i32> = HashSet::new();
    let mut found_path:bool = false;
    for sp1 in 0..n {
        for sp2 in 0..n {
            if sp1 == sp2 {
                continue;
            } else {
                visited.insert(sp1);
                stack.push(sp1);
                visited.insert(sp2);
                stack.push(sp2);
                let mut step_backwards:bool = false;
                let mut last:i32 = -1; 
                loop {
                    if step_backwards && stack.len() == 2 {
                        break;
                    } else if step_backwards {
                        last = stack.pop().unwrap();
                        visited.remove(&last);
                    } else {
                        let mut closest_distance:f32 = 0.0;
                        let mut closest_number:i32 = n;
                        for i in (last+1)..n {
                            let lp:usize = stack[stack.len()-1] as usize;
                            let llp:usize = stack[stack.len()-2]as usize;
                            if aa[llp][lp][i as usize] {
                                if closest_number == n {
                                    closest_number = i;
                                    closest_distance = ad[lp][i as usize];
                                } else {
                                    let distance:f32 = ad[lp][i as usize];
                                    if distance < closest_distance {
                                        closest_number = i;
                                        closest_distance = ad[lp][i as usize];
                                    }
                                }
                            }
                        }
                        if closest_number == n {
                            step_backwards = true;
                        } else {
                            // auf stack legen in visited einfügen last auf null setzten 
                        }
                    }
                }
                if found_path {break;}
                visited.remove(&sp1);
                stack.pop();
                visited.remove(&sp2);
                stack.pop();
            }
            if found_path {break;}
        }
        if found_path {break;}
    }
    stack
}

