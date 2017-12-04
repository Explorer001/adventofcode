use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    println!("{}", get_valid(false));
    println!("{}", get_valid(true));
    //println!("{}", same("foo".to_string(), "ofo".to_string()));
}

fn same(element: String, mut reference: String)-> bool {
    //println!("({}, {})", element, reference);
    if element.len() != reference.len() {return false;}
    //println!("({}, {})", element, reference);
    let buf: String = String::new();
    for i in 0..element.len() {
        for j in 0..reference.len() {
            if element.chars().nth(i).unwrap() == reference.chars().nth(j).unwrap() {
                reference.remove(j);
                break;
            }
        }
    }
    //println!("({}, {}) | {}", element, reference, reference.len()==0);
    return reference.len() == 0;
}

fn get_valid(part2: bool) -> i32 {
    let file = File::open("puzzleinput").expect("open failed");
    let buf_read = BufReader::new(file);
    let mut split: Vec<String>;
    let mut valid_count: i32 = 0;
    let mut valid: bool;
    let mut phrase_vec: Vec<String> = Vec::new();
    let mut temp: String = String::new();
    for line in buf_read.lines() {
        valid = true;
        split = line.unwrap().split(' ').map(|s| s.to_string()).collect();
        for element in split {
            for x in 0..phrase_vec.len() {
                temp = (*phrase_vec.get(x).unwrap()).clone();
                if part2 {
                    if same(element.to_string(), temp){
                        valid = false;
                        continue;
                    }
                } else {
                    if element == temp {
                        valid = false;
                        continue;
                    }
                }
            }
            phrase_vec.push(element);
        }
        if valid {valid_count += 1;}
        phrase_vec.clear();
    }
    return valid_count;
}
