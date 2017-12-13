use std::io;
use std::io::{BufReader, BufRead};

fn main() {
    //stepcount = (range - 1)*2
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<i32>> = buf_reader.lines()
                                         .flat_map(Result::ok)
                                         .map(|s| s.split(": ")
                                                   .map(|p| p.to_string()
                                                             .parse()
                                                             .unwrap())
                                                   .collect())
                                         .collect();

    println!("{}", get_severity(input.to_vec(), 0));
    println!("{}", get_delay(input.to_vec()));
}

fn get_delay(input: Vec<Vec<i32>>) -> i32 {
    let mut delay: i32 = 0;
    while get_severity(input.to_vec(), delay) != 0 {
        delay += 1;
    }
    delay
}

fn get_severity(input: Vec<Vec<i32>>, delay: i32) -> i32 {
    let mut depth: i32;
    let mut range: i32;
    let mut severity: i32 = 0;
    for element in &input {
        depth = element[0] + delay;
        range = element[1];
        if depth%((range-1)*2) == 0 {severity += depth*range}
    }
    severity
}
