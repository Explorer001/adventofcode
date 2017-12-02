use std::fs::File;
use std::io::{BufReader, BufRead};
use std::vec::Vec;
use std::cmp;

fn main() {
    let file = File::open("puzzleinput").expect("open failed");
    let buf_read = BufReader::new(file);
    let mut line_vec = Vec::new();
    let mut temp: i32 = 0;
    let mut checksum: i32 = 0;
    let mut checksum2: i32 = 0;
    let mut linemax: i32 = 0;
    let mut linemin: i32 = 0;
    let mut got_char: bool = false;
    let mut num_str = String::new();
    let mut ln = String::new();
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for line in buf_read.lines() {
        ln = line.unwrap();
        ln.push('$');
        linemax = 0;
        linemin = <i32>::max_value();
        for c in ln.chars() {
            if c != ' ' && c != '$' {
                got_char = true;
                num_str.push(c);
                //println!("{}", num_str);
            }
            else if (c == ' ' || c == '$') && got_char == true {
                got_char = false;
                temp = num_str.parse().unwrap();
                line_vec.push(temp);
                num_str.clear();
                if temp > linemax {linemax = temp;}
                if temp < linemin {linemin = temp;}
            }
        }
        for x in 0..line_vec.len() {
            for y in 0..line_vec.len() {
                a = *line_vec.get(x).unwrap();
                b = *line_vec.get(y).unwrap();
                if a%b == 0 && x != y {
                    checksum2 += cmp::max(a,b)/cmp::min(a,b);
                    break;
                }
            }
        }
        line_vec.clear();
        checksum += linemax - linemin;
    }
    println!("{}, {}", checksum, checksum2);
}
