use std::str;
use std::io;
use std::io::{BufRead, BufReader};

const ASCII_A: u8 = 97;

fn main() {

    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<String> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|l| l.to_string())
                                            .collect();

    run(input.to_vec());
}

fn run(instructions: Vec<String>) {
    
    let mut position: usize = 0;

    let mut inst: Vec<String>;

    let mut reg_names: Vec<u8> = Vec::new();
    let mut register_values: Vec<i64> = vec![0; 26];
    
    let mut last_snd: i64 = 0;

    let mut first_reg_index: usize;
    let mut second_inst;
    let mut second_value: i64 = 0;

    for i in 0..26 {
        reg_names.push(ASCII_A + (i as u8));
    }

    let register_names: Vec<String> = str::from_utf8(&reg_names).unwrap()
                                                                .to_string()
                                                                .chars()
                                                                .map(|c| c.to_string())
                                                                .collect();

    
    while position < instructions.len() {
        let element = instructions[position].to_string();
        inst = element.split_whitespace()
                      .map(|s| s.to_string())
                      .collect();
        first_reg_index = register_names.iter().position(|p| p.to_string() == inst[1]).unwrap();

        if inst.len() > 2 {
            second_inst = inst[2].parse::<i64>();
            match second_inst {
                Ok(val) => second_value = val,
                Err(_why) => second_value = register_values[register_names.iter().position(|p| p.to_string() == inst[2]).unwrap()],
            }
        }

        if inst[0] == "snd" {
            last_snd = register_values[first_reg_index];
            println!("Playing {}! BingBong!", last_snd);
        } else if inst[0] == "set" {
            register_values[first_reg_index] = second_value;
        } else if inst[0] == "add" {
            register_values[first_reg_index] += second_value;
        } else if inst[0] == "mul" {
            register_values[first_reg_index] *= second_value;
        } else if inst[0] == "mod" {
            register_values[first_reg_index] = register_values[first_reg_index] % second_value;
        } else if inst[0] == "rcv" {
            if register_values[first_reg_index] != 0 {
                register_values[first_reg_index] = last_snd;
                println!("Recovered {}!", last_snd);
                return;
            }
        } else if inst[0] == "jgz" {
            if register_values[first_reg_index] > 0 {
                position = ((position as i64) + second_value) as usize;
                continue;
            }
        }
        position += 1;
    }
}
