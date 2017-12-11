use std::io;
use std::io::{BufRead, BufReader};
use std::cmp;

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|s| s.split(",")
                                                      .map(|s| s.to_string())
                                                      .collect())
                                            .collect();
    let (x,y,z,max_dist) = get_coords(input[0].to_vec());
    println!("Distance: {}, Max Distance: {}", dist((0,0, 0), (x,y,z)), max_dist);
}

fn dist(p1: (i32, i32, i32), p2: (i32, i32, i32)) -> i32 {
    let (x1,y1,z1) = p1;
    let (x2,y2,z2) = p2;
    cmp::max(cmp::max((x1-x2).abs(), (y1-y2).abs()), (z1-z2).abs())
}

fn get_coords(path: Vec<String>) -> (i32, i32, i32, i32) {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut z: i32 = 0;
    let mut max_dist: i32 = 0;
    let mut temp: i32;
    for element in &path {
        if *element == "n" {
            x += 1;
            y -= 1;
        } else if *element == "ne" {
            x += 1;
            z -= 1;
        } else if *element == "nw" {
            y -= 1;
            z += 1;
        } else if *element == "s" {
            x -= 1;
            y += 1;
        } else if *element == "se" {
            y += 1;
            z -= 1;
        } else if *element == "sw" {
            x -= 1;
            z += 1;
        }
        temp = dist((0,0,0), (x,y,z));
        if temp > max_dist {max_dist = temp;}
    }

    (x, y, z, max_dist)
}
