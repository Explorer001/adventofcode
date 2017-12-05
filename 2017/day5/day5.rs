use std::io;
use std::io::{BufReader, BufRead};

fn main() {
    let buf_read = BufReader::new(io::stdin());
    let mut input: Vec<i32> = buf_read.lines()
                                  .flat_map(Result::ok)
                                  .map(|s| s.to_string().parse().unwrap())
                                  .collect();
    let mut cp = input.to_vec();
    println!("Part1: {}", solve(&mut input, false));
    println!("Part2: {}", solve(&mut cp, true));
}

fn solve(input: &mut Vec<i32>, part: bool) -> i32 {
    let mut i: i32 = 0;
    let mut temp: i32;
    let mut steps = 0i32;
    //println!("{:?}", input);
    while i >= 0 && i < input.len() as i32{
        //println!("{:?}, {}", input, i);
        temp = input[i as usize];
        if part && temp.abs() >= 3{
            if temp < 0 {
                input[i as usize] += 1;
            } else {
                input[i as usize] -= 1;
            }
        } else {
            input[i as usize] += 1;
        }
        i += temp;
        steps += 1;
    }
    steps
}
