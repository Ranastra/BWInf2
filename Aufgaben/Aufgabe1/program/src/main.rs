use std::f32::consts::PI;
use std::f32;
use std::str::FromStr;
use std::io::{BufReader, BufRead};
use std::fs::File;

const HALF_PI:f32 = PI/2.0;

fn main() {
    //println!("Hello, world!");
    println!("{}", distance(0.0, 0.0, 1.0, 1.0));
    //let mut a = new String;
    //let two:f32 = 2.0;
    //println!("{}", two.sqrt());
    //println!("{}", angle(1.0,1.0,1.4));
    //println!("{}", HALF_PI);
    let (n, test1) = read_input(1);
    //println!("{}, {:?}", n, test1);
    get_all_distances(n, &mut test1);
}

fn distance(x0:f32, y0:f32, x1:f32, y1:f32) -> f32 {
    let difx:f32 = x0-x1;
    let dify:f32 = y0-y1;
    (difx*difx + dify*dify).sqrt()
}

fn angle(d1:f32, d2:f32, d3:f32) -> f32 {
    let cosa:f32 = (d1*d1 + d2*d2 - d3*d3)/(2.0*d1*d2);
    cosa.acos()
}


fn read_input(number_of_test: i32) -> (u32, Vec::<[f32; 2]>) {
    let s:String = format!("../testcases/bsp{}.txt", number_of_test.to_string());
    let file:File = File::open(s).unwrap();
    let mut all_points:Vec<[f32; 2]> = Vec::<[f32; 2]>::new();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut n: u32 = 0;
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

fn get_all_distances(n:u32, all_points:&mut Vec::<[f32;2]>) {
    //let mut all_distances: [[f32; n]; n] = [[0.0; n]; n];
    let mut all_distances:Vec<Vec<[f32;2],2>,2> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in n..0 {
            if j == i {
                break;
            } else {
                println!("{}, {}", i, j);
            }
        }
    }
}


