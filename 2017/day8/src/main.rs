use std::io;
use std::io::{BufReader, BufRead};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|s| s.split_whitespace()
                                                      .filter(|&s| s != "if")
                                                      .map(|s| s.to_string())
                                                      .collect())
                                            .collect();
    println!("{:?}", run(input.to_vec()));
}

fn run(input: Vec<Vec<String>>) -> (i32, i32) {
    let mut registers: Vec<String> = Vec::new();
    let mut reg_value: Vec<i32> = Vec::new();
    let mut real_max: i32 = <i32>::min_value();
    for line in &input {
        if !registers.contains(&line[0]) {
            registers.push(line[0].to_string());
            reg_value.push(0i32);
        }
    }
    for line in &input {
        let mut re_val: i32 = 0;
        let mut cond: bool = false;
        let mut alter_index = 0;
        for i in 0..registers.len() {
            if registers[i] == line[3] {
                re_val = reg_value[i];
            } 
            if registers[i] == line[0] {
                alter_index = i;
            }
        }
        let operator = line[4].to_string();
        let value: i32 = line[5].parse().unwrap();
        if operator == "==" {
            if re_val == value {cond = true;}
        } else if operator == "!=" {
            if re_val != value {cond = true;}
        } else if operator == ">=" {
            if re_val >= value {cond = true;}
        } else if operator == "<=" {
            if re_val <= value {cond = true;}
        } else if operator == "<" {
            if re_val < value {cond = true;}
        } else if operator == ">" {
            if re_val > value {cond = true;}
        }
        let alter_value: i32 = line[2].parse().unwrap();
        if cond {
            if line[1] == "dec" {
                reg_value[alter_index] -= alter_value;
            } else if line[1] == "inc" {
                reg_value[alter_index] += alter_value;
            }
        }
        for reg in &reg_value {
            if reg > &real_max {real_max = *reg;}
        }
    }
    let mut max: i32 = <i32>::min_value();
    for reg in &reg_value {
        if reg > &max {max = *reg;}
    }
    (max, real_max)
}
