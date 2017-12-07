use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|s| s.split_whitespace()
                                                      .filter(|&s| s != "->")
                                                      .map(|s| s.replace(",","")
                                                                .replace("(","")
                                                                .replace(")","")
                                                                .to_string())
                                                      .collect())
                                            .collect();
    //println!("{:?}", input);

    let root = get_root(input.to_vec());
    println!("Root: {}", root);
    let foo = get_unbalanced(input.to_vec(), root.to_string());
    println!("{}", foo);
}

fn get_unbalanced(input: Vec<Vec<String>>, node: String) -> i32{
    for i in 0..input.len() {
        if input[i][0] == node {
            if input[i].len() == 2 {
                //println!("{:?}", input[i]);
                return input[i][1].parse().unwrap();
            } else {
                let mut sol: Vec<i32> = Vec::new(); 
                for j in 2..input[i].len() {
                    sol.push(get_unbalanced(input.to_vec(), input[i][j].to_string()));
                }
                //println!("{:?} ||| {:?}", input[i], sol);
                let mut unbal = false;
                for k in 0..sol.len()-1 {
                    if sol[k] != sol[k+1]  {
                        unbal = true;
                        break;
                    }
                }
                if unbal {
                    let mut max_ind = 0;
                    let mut max = 0;
                    for i in 0..sol.len() {
                        if sol[i] > max {
                            max = sol[i];
                            max_ind = i;
                        }
                    }
                    let unbal_program = input[i][max_ind + 2].to_string();
                    let mut unbal_weight = 0;
                    for g in 0..input.len() {
                        if input[g][0] == unbal_program {
                            unbal_weight = input[g][1].parse::<i32>().unwrap();
                            break;
                        }
                    }
                    println!("--------------------------------------");
                    println!("{:?}", sol);
                    println!("{} disk is not balanced", input[i][0]);
                    println!("{:?}", input[i]);
                    let max: &i32 = sol.iter().max().unwrap();
                    let min: &i32 = sol.iter().min().unwrap();
                    println!("{} weight must be {}", unbal_program, unbal_weight - (*max - *min));
                    println!("--------------------------------------");
                }
                return input[i][1].parse::<i32>().unwrap() + sol.iter().sum::<i32>();
            }
        }
    }
    return 0;
}

fn get_root(input: Vec<Vec<String>>) -> String {
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

    name                        
}
