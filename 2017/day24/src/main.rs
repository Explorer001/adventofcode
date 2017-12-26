use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<String> = buf_reader.lines()
                                       .flat_map(Result::ok)
                                       .map(|l| l.to_string())
                                       .collect();
    get_max(input.to_vec());
    unsafe {
        println!("{}", STRONGEST_WEIGHT);
        println!("{}", LONGEST_WEIGHT);
    }
}

fn get_max(input: Vec<String>) {

    let mut start_node: String;
    //get start node(s)
    for element in &input {
        let parts: Vec<String> = element.split("/").map(str::to_string).collect();
        if parts[0] == "0" || parts[1] == "0" {
            start_node = element.to_string();
            println!("----Using: {}----", start_node);
            get_max_helper(vec![start_node], input.to_vec());
        }
    }
}

static mut STRONGEST_WEIGHT: u64 = 0;
static mut LONGEST_BRIDGE: usize = 0;
static mut LONGEST_WEIGHT: u64 = 0;


fn get_max_helper(bridge: Vec<String>, parts: Vec<String>) {
    let mut bridge_parts = parts.to_vec();
    
    //println!("{:?}", bridge);
    //get weight of bridge
    let weight = get_weight(bridge.to_vec());

    //remove new element of bridge from parts
    for i in 0..bridge_parts.len() {
        if &bridge_parts[i] == bridge.last().unwrap() {
            bridge_parts.remove(i);
            break;
        }
    }

    unsafe {
        if weight > STRONGEST_WEIGHT {
            STRONGEST_WEIGHT = weight;
            println!("{}", STRONGEST_WEIGHT);
            println!("{:?}", bridge);
            println!("{:?}", bridge_parts);
            println!("");
        }
        if bridge.len() > LONGEST_BRIDGE {
            LONGEST_BRIDGE = bridge.len();
            LONGEST_WEIGHT = weight;
        } else if bridge.len() == LONGEST_BRIDGE {
            if weight > LONGEST_WEIGHT {LONGEST_WEIGHT = weight;}
        }
    }

    //generate new bridges
    let new_docker: Vec<String> = bridge.last().unwrap()
                                        .split("/").map(str::to_string).collect();

    let mut candidate: Vec<String>;
    let mut new_bridge: Vec<String>;
    let mut nm2: Vec<String>;

    for i in 0..bridge_parts.len() {
        new_bridge = bridge.to_vec();
        candidate = bridge_parts[i].split("/").map(str::to_string).collect();
        if bridge.len() > 1 {
           nm2 = bridge[bridge.len()-2].split("/").map(str::to_string).collect();
           if new_docker[0] == nm2[0] || new_docker[0] == nm2[1] {
                if candidate[0] == new_docker[1] || candidate[1] == new_docker[1] {
                    new_bridge.push(bridge_parts[i].to_string());
                    get_max_helper(new_bridge.to_vec(), bridge_parts.to_vec());
                }
            } else if new_docker[1] == nm2[0] || new_docker[1] == nm2[1] {
                if candidate[0] == new_docker[0] || candidate[1] == new_docker[0] {
                    new_bridge.push(bridge_parts[i].to_string());
                    get_max_helper(new_bridge.to_vec(), bridge_parts.to_vec());
                }
            }
        }  else {
            if new_docker[0] == "0" {
                if candidate[0] == new_docker[1] || candidate[1] == new_docker[1] {
                    new_bridge.push(bridge_parts[i].to_string());
                    get_max_helper(new_bridge.to_vec(), bridge_parts.to_vec());
                }
            } else {
                if candidate[0] == new_docker[0] || candidate[1] == new_docker[0] {
                    new_bridge.push(bridge_parts[i].to_string());
                    get_max_helper(new_bridge.to_vec(), bridge_parts.to_vec());
                }
            }
        }
    }

}

fn get_weight(bridge: Vec<String>) -> u64 {
    let mut weight: u64 = 0;
    let mut weights: Vec<u64>;
    for element in &bridge {
        weights = element.split("/")
                         .map(|s| s.to_string().parse().unwrap())
                         .collect();
        weight += weights[0]+weights[1];
    }
    weight
}
