use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|s| s.split_whitespace()
                                                      .filter(|&s| s != "->")
                                                      .map(|s| s.replace(",","")
                                                                .to_string())
                                                      .collect())
                                            .collect();
    
    let mut elementi: Vec<String>;
    let mut elementj: Vec<String>;
    let mut name: String = "foo".to_string();
    let mut refed: bool;
    let mut brk: bool;
    for i in 0..input.len() {
        refed = false;
        elementi = input[i].to_vec();
        name = elementi[0].to_string();
        if elementi.len() < 3 {
            continue;
        }
        for j in 0..input.len() {
            brk = false;
            if i != j {
                elementj = input[j].to_vec();
                //println!("{:?} <> {:?}", elementi, elementj);
                for k in 2..elementj.len() {
                    if elementj[k] == name {
                        refed = true;
                        brk = true;
                        break;
                    }
                }
            }
            if brk {break;}
        }
        if !refed {
            break;
        }
    }

    println!("{}", name);
                         
}
