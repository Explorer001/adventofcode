use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<i32>> = buf_reader.lines()
                                        .flat_map(Result::ok)
                                        .map(|s| s.split_whitespace()
                                                  .filter(|&p| p != "<->")
                                                  .map(|p| p.replace(",","")
                                                            .to_string()
                                                            .parse()
                                                            .unwrap())
                                                  .collect())
                                        .collect();
    let con: i32 = get_num_connected_to(input.to_vec(), 0);
    println!("{}", con);
}

fn get_num_connected_to(input: Vec<Vec<i32>>, num: i32) -> i32{
    let connected = connected_to(num, input.to_vec());
    connected.len() as i32
}

fn connected_to(num: i32, input: Vec<Vec<i32>>) -> Vec<i32> {
    let mut con: Vec<i32> = Vec::new();
    let mut old_len: usize;
    for _ in 0..input.len() {
        old_len = con.len();
        for element in &input {
            if element[0] == num  || con.contains(&element[0]){
                for i in 0..element.len() {
                    if !con.contains(&element[i]) {
                        con.push(element[i]);
                    }
                }
            }
        }
        if old_len ==  con.len() {
            break;
        }
    }
    con
}
