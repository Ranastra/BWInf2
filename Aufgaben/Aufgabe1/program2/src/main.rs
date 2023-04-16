use std::f64::consts::PI;
use std::f64;
use std::str::FromStr;
use std::io::{BufReader, BufRead, Write};
use std::fs::File;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::env;

const PRINT_STEPS:bool = true; // print all steps 
const PRINT_TIME_ALL:bool = true; // prints time for one testcase
const PRINT_TIME_TOTAL:bool = true; // prints total time of all test
const DEBUG_MODE:bool = true; // enable debug mode to run with cargo
const DISPLAY:bool = false; // display steps in pathfinder loop

const HALF_PI:f64 = PI/2.0; // 90°

fn main() {
    let start_time_all:Instant = Instant::now();
    let args: Vec<String> = env::args().collect(); // from cl passed arguments
    let mut paths_and_names:Vec<(String, String)> = Vec::new(); // stores the names and paths of the testcases
    if !DEBUG_MODE {
        // map parameters
        let path:String = args[1].clone();
        let name:String = args[1].rsplit("/").next().unwrap().to_string();
        if name == "" {
            // quit if no testfile is given
            println!("give the relative path to a testcase!");
            return;
        } else if name == "all" {
            // run for all testcases
            for i in 1..8 {
                paths_and_names.push((format!("testcases/bsp{i}.txt"), format!("bsp{i}.txt")));
            }
        } else {
            // run for specified testcase
            paths_and_names.push((path, name));
        }
    } else {
        // test with cargo run
        for i in 5..6 {
            paths_and_names.push((format!("../testcases/bsp{i}.txt"), format!("bsp{i}.txt")));
        }
        // rand_mode = false;
        // fast_mode = true;
    }
    for (path, name) in paths_and_names {
        let start_time:Instant = Instant::now();
        if PRINT_STEPS {println!("test: {}", path)}
        // read input
        let (n, points) = read_input(path);
        if PRINT_STEPS {println!("finish reading input");}
        // convert to paths
        let converted_paths = convert(n as usize, points);
        // solve
        let found_path:Option<Path> = solve(n as usize, converted_paths);
        if PRINT_STEPS {println!("finish solve")}
        // output path
        match found_path {
            Some(path) => {
                path.output(name);
                if PRINT_STEPS {println!("found solution");}
            },
            None => {
                Path::new_empty().output(name);
                if PRINT_STEPS {println!("found no solution");}
            },
        }
        if PRINT_STEPS {println!("finish output")}
        if PRINT_TIME_ALL {
            let timedelta:Duration = start_time.elapsed();
            let time:f64 = timedelta.as_secs_f64();
            println!("Gesammte Zeit für den Test in Sekunden: {}\n", time);
        }
    }
    if PRINT_TIME_TOTAL {
        let timedelta:Duration = start_time_all.elapsed();
        let time:f64 = timedelta.as_secs_f64();
        println!("Zeit für alle Tests in Sekunden: {}", time);
    }
}

struct Point(f64, f64);

impl Point {
    fn new(x:f64, y:f64) -> Point {
        Point(x, y)
    }

    fn distance(&self, other:&Point) -> f64 {
        // calculate the distance with pythagoras
        ((self.0-other.0).powi(2) + (self.1-other.1).powi(2)).sqrt()
    }

    fn angle(&self, other0:&Point, other1:&Point) -> bool {
        // calculate if the angle is over 90° with law of cosines
        let d1:f64 = self.distance(other0);
        let d2:f64 = self.distance(other1);
        let d3:f64 = other0.distance(other1);
        let cosa:f64 = (d1*d1 + d2*d2 - d3*d3)/(2.0*d1*d2);
        cosa.acos() >= HALF_PI
    }
}

struct Modification {
    position: usize,
    modified_id: usize,
    other_id: usize,
    distance: f64,
    other_len: usize,
}

impl Modification {
    fn new(position:usize, modified_id:usize, other_id:usize, distance:f64, other_len:usize) -> Modification {
        Modification { position, modified_id, other_id, distance, other_len }
    }

    fn new_empty() -> Modification {
        Modification::new(0, 0, 0, 0.0, 0)
    }

    fn is_before(&self, other:&Modification) -> bool {
        // compares two modifikation by distances, modified_id, other_id, position
        // if the same return false
        if self.distance < other.distance {
            true
        } else if self.distance > other.distance {
            false
        } else {
            if self.modified_id < other.modified_id {
                true
            } else if self.modified_id > other.modified_id {
                false
            } else {
                if self.other_id < other.other_id {
                    true
                } else if self.other_id > other.other_id {
                    false
                } else {
                    if self.position < other.position {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }

    fn _show(&self) {
        println!("pos: {}, mID: {}, oID: {}, dis: {}, len: {}", self.position, self.modified_id, self.other_id, self.distance, self.other_len);
    }
}

struct Path {
    points:Vec<Point>,
    id:usize,
}

impl Path {
    fn new(point:Point, id:usize) -> Path {
        Path {
            points : vec![point],
            id,
        }
    }

    fn new_empty() -> Path {
        Path {
            points : Vec::new(),
            id : 0,
        }
    }

    fn connect(&mut self, mut other:Path, modification:&Modification) {
        if modification.position / 2 == 1 {
            other.points.reverse();
        }
        if modification.position % 2 == 1 {
            std::mem::swap(&mut self.points, &mut other.points);
        }
        self.points.append(&mut other.points);
    }

    fn undo(&mut self, modification:&Modification) -> Path {
        let splitoff_len:usize;
        if modification.position % 2 == 0 {
            splitoff_len = self.points.len() - modification.other_len;
        } else {
            splitoff_len = modification.other_len;
        }
        let mut other_path = self.points.split_off(splitoff_len);
        if modification.position % 2 == 1 {
            std::mem::swap(&mut self.points, &mut other_path);
        }
        if modification.position / 2 == 1 {
            other_path.reverse();
        }
        Path { points: other_path, id: modification.other_id }
    }

    fn get_connect(&self, other:&Path) -> Vec<Modification> {
        if self.len() == 1 && other.len() == 1 {
            vec![Modification::new(0, self.id, other.id, self.get(0).distance(other.get(0)),other.len())]
        } else if self.len() == 1 {
            let mut conns:Vec<Modification> = Vec::with_capacity(2);
            let self0 = self.get(0);
            let other0 = other.get(0);
            let othern1 = other.get(-1);
            if other0.angle(self0, other.get(1)) {
                conns.push(Modification::new(0, self.id, other.id, self0.distance(other0), other.len()));
            }
            if othern1.angle(self0, other.get(-2)) {
                conns.push(Modification::new(1, self.id, other.id, self0.distance(othern1), other.len()));
            }
            conns
        } else if other.len() == 1 {
            let mut conns:Vec<Modification> = Vec::with_capacity(2);
            let other0 = other.get(0);
            let self0 = self.get(0);
            let selfn1 = self.get(-1);
            if self0.angle(self.get(1), other0) {
                conns.push(Modification::new(1, self.id, other.id, other0.distance(self0), other.len()));
            }
            if selfn1.angle(self.get(-2), other0) {
                conns.push(Modification::new(0, self.id, other.id, selfn1.distance(other0), other.len()));
            }
            conns
        } else {
            let mut conns:Vec<Modification> = Vec::with_capacity(4);
            let self0 = self.get(0);
            let self1 = self.get(1);
            let selfn1 = self.get(-1);
            let selfn2 = self.get(-2);
            let other0 = other.get(0);
            let other1 = other.get(1);
            let othern1 = other.get(-1);
            let othern2 = other.get(-2);
            if selfn1.angle(selfn2, other0) &&
               other.get(0).angle(selfn1, other1) {
                conns.push(Modification::new(0, self.id, other.id, selfn1.distance(other0), other.len()));
               }
            if othern1.angle(othern2, self0) &&
               self0.angle(othern1, self1) {
                conns.push(Modification::new(1, self.id, other.id, othern1.distance(self0), other.len()));
               }
            if selfn1.angle(selfn2, othern1) && 
               othern1.angle(othern2, selfn1) {
                conns.push(Modification::new(2, self.id, other.id, selfn1.distance(othern1), other.len()));
               }
            if self0.angle(self1, other0) &&
               other0.angle(other1, self0) {
                conns.push(Modification::new(3, self.id, other.id, self0.distance(other0), other.len()));
               }
            conns
        }
    }

    fn get_at(&self, mut index:i32) -> Option<&Point> {
        if index < 0 {
            index = (self.points.len() as i32) + index;
        } 
        if index < 0 || index >= (self.points.len() as i32) {
            None
        } else {
            Some(&self.points[index as usize])
        }
    }

    fn get(&self, index:i32) -> &Point {
        self.get_at(index).unwrap()
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn output(self, name:String) {
        let mut total_distance:f64 = 0.0;
        if self.len() > 1 {
            for i in 0..(self.len() -1) as i32 {
                total_distance += self.get(i).distance(self.get(i+1));
            }
        }
        let mut output:String = String::from(format!("{}\n", total_distance));
        for point in self.points {
            let (x, y) = (point.0, point.1);
            output.push_str(&format!("{}, {}\n", x, y));
        }
        let path: String;
        if !DEBUG_MODE {
            path = format!("output/{}", name);
        } else {
            path = format!("../output/{}", name);
        }
        let mut file = File::create(path).unwrap();
        file.write_all(output.as_bytes()).unwrap();
    }

    fn _show(&self) {
        let mut s:Vec<(f64, f64)> = Vec::with_capacity(self.len());
        for point in &self.points {
            s.push((point.0, point.1));
        }
        println!("Path: {} || [{:?}]", self.id, s);
    }
}

fn convert(n:usize, points:Vec<[f64; 2]>) -> HashMap<usize, Path> {
    let mut paths:HashMap<usize, Path> = HashMap::with_capacity(n);
    for i in 0..points.len() {
        let [x, y] = points[i];
        paths.insert(i, Path::new(Point::new(x, y), i));
    }
    paths
}

fn read_input(path: String) -> (i32, Vec<[f64; 2]>) {
    // gets the content from a file at an absolute path
    let file:File = File::open(path).unwrap();
    let mut all_points:Vec<[f64; 2]> = Vec::new();
    let reader:BufReader<File> = BufReader::new(file);
    let lines = reader.lines();
    let mut n: i32 = 0;
    // read and convert the koordinates
    let mut x:f64;
    let mut y:f64;
    for line in lines {
        let line:String = line.unwrap();
        let mut line2 = line.trim().split_whitespace();
        x = f64::from_str(line2.next().unwrap()).unwrap();
        y = f64::from_str(line2.next().unwrap()).unwrap();
        all_points.push([x, y]);
        n +=1;
    };
    (n, all_points)
}

fn solve(n:usize, mut paths:HashMap<usize, Path>) -> Option<Path> {
    let mut best_option:Modification;
    let mut step_backwards:bool = false;
    let mut stack:Vec<Modification> = Vec::with_capacity(n);
    let mut last_choosen_mod:Modification = Modification::new_empty(); 
    let mut best_is_set:bool;
    loop {
        if DISPLAY {println!("");}
        if paths.len() == 1 {
            return Some(paths.drain().next().unwrap().1);
        }
        if !step_backwards {
            best_option = Modification::new_empty();
            best_is_set = false;
            for i in paths.keys() {
                for j in paths.keys() {
                    if i <= j {
                        continue;
                    }
                    for option in paths[&i].get_connect(&paths[&j]) {
                        if !last_choosen_mod.is_before(&option) {
                            continue;
                        } else if !best_is_set {
                            best_is_set = true;
                            best_option = option;
                        } else if option.is_before(&best_option) {
                            best_option = option;
                        }
                    }
                }
            }
            if DISPLAY {println!("best_is_set: {}", best_is_set);}
            if best_is_set {
                let choosen_other_path:Path = paths.remove(&best_option.other_id).unwrap();
                let choosen_mod_path:&mut Path = paths.get_mut(&best_option.modified_id).unwrap();
                choosen_mod_path.connect(choosen_other_path, &best_option);
                if DISPLAY {
                    print!("best_option: ");
                    best_option._show();
                }
                stack.push(best_option);
                last_choosen_mod = Modification::new_empty();
            } else {
                step_backwards = true;
            }
        } else {
            if stack.len() == 0 {
                return None;
            }
            step_backwards = false;
            let last_modifikation = stack.pop().unwrap();
            if DISPLAY {
                print!("backstep\nlast_modifikation: ");
                last_modifikation._show();
            }
            let modified_path = paths.get_mut(&last_modifikation.modified_id).unwrap();
            let phoenix_path = modified_path.undo(&last_modifikation);
            last_choosen_mod = last_modifikation;
            paths.insert(last_choosen_mod.other_id, phoenix_path);
        }
        if DISPLAY {
            println!("stack_len: {}, path_len: {}, backwards: {}", stack.len(), paths.len(), step_backwards);
        }
    }
}
