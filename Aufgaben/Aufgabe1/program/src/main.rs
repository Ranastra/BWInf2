//use std::f32;
use std::f32::consts::PI;

const HALF_PI:f32 = PI/2.0;

fn main() {
    //println!("Hello, world!");
    println!("{}", distance(0.0, 0.0, 1.0, 1.0));
    //let mut a = new String;
    let two:f32 = 2.0;
    println!("{}", two.sqrt());
    println!("{}", angle(1.0,1.0,1.4));
    println!("{}", HALF_PI);
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