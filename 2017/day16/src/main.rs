use std::str;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    //let input: Vec<String> = vec!["s1".to_string(),"x3/4".to_string(),"pe/b".to_string()];
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|s| s.split(",")
                                                      .map(|e| e.to_string())
                                                      .collect())
                                            .collect();
    //println!("{:?}", input);
    println!("{}", solve(input[0].to_vec(), 16));
}

fn solve(input: Vec<String>, list_size: usize) -> String {

    let mut instruction: Vec<String>;
    
    let utf8_a: u8 = 97;
    let mut char_buf: Vec<u8> = Vec::new();

    for i in 0..list_size {
        char_buf.push(utf8_a + (i as u8));
    }
    let mut program_list: Vec<String> = str::from_utf8(&char_buf).unwrap()
                                                                 .to_string()
                                                                 .chars()
                                                                 .map(|c| c.to_string())
                                                                 .collect();


    let mut shift_size: usize;

    let mut index_a: usize;
    let mut index_b: usize;

    let mut vector_element: String;
    let mut operand: String = String::new();

    for element in &input {
        instruction = element.split("/")
                             .map(|c| c.to_string())
                             .collect();
        let mut string_1: String = String::new();
        let mut count: i32 = 0;
        for i in instruction[0].chars() {
            if count > 0 {
                string_1.push(i);
            } else {
                operand = i.to_string();
            }
            count += 1;
        }

        if operand == "s" {
            shift_size = string_1.parse().unwrap();
            for _ in 0..shift_size {
                vector_element = program_list.pop().unwrap();
                program_list.insert(0, vector_element); 
            }
        } else if operand == "x" || operand == "p" {
            if operand == "x" {
                index_a = string_1.parse().unwrap();
                index_b = instruction[1].parse().unwrap();
            } else {
                index_a = program_list.iter().position(|r| r.to_string() == string_1).unwrap();
                index_b = program_list.iter().position(|r| r.to_string() == instruction[1]).
                        unwrap();
            }

            program_list.swap(index_a, index_b);
        }
        //println!("{}", program_list.iter().map(|e| e.to_string()).collect::<String>());
    }

    program_list.iter().map(|e| e.to_string()).collect()
}
