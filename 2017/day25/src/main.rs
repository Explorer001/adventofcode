use std::io;
use std::io::{BufReader, BufRead};

//length of one state description
const STATE_LEN: usize = 9;

fn main() {
    let input = parse_input();
    let tm = create_tm("foo".to_string(), input.to_vec());
    println!("{}", run_tm(tm, input[0].to_string(), input[1].parse().unwrap()));
}

#[derive(Debug, Clone)]
struct Operation {
    write: u8,
    move_direction: String,
    next_state: String,
}

#[derive(Debug, Clone)]
struct State {
    name: String,
    operation_zero: Operation,
    operation_one: Operation,
}

 #[derive(Debug, Clone)]
struct TuringMachine {
    name: String,
    states: Vec<State>,
    band: Vec<u8>,
}

fn parse_input() -> Vec<String> {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<String> = buf_reader.lines()
                                       .flat_map(Result::ok)
                                       .map(|l| l.to_string())
                                       .collect();
    
    let mut parsed_input: Vec<String> = Vec::new();
    let mut line_parts: Vec<String>;

    for element in &input {
        //ignore blank lines in input
        if element == "" {continue};
        //split line for better handling
        line_parts = element.split_whitespace()
                            .filter(|&x| x != "-")
                            .map(|p| p.to_string())
                            .collect();
        
        if line_parts[0] == "Begin" {
            parsed_input.push(line_parts[3].replace(".", ""));
        } else if line_parts[0] == "Perform" {
            parsed_input.push(line_parts[5].to_string());
        } else if line_parts[0] == "If" {
            parsed_input.push(line_parts[5].replace(":", ""));
        } else if line_parts[0] == "In" {
            parsed_input.push(line_parts[2].replace(":", ""));
        } else if line_parts[0] == "Move" {
            parsed_input.push(line_parts[5].replace(".", ""));
        } else if line_parts[0] == "Write" {
            parsed_input.push(line_parts[3].replace(".", ""));
        } else if line_parts[0] == "Continue" {
            parsed_input.push(line_parts[3].replace(".", ""));
        }
    }

    parsed_input
}

fn create_tm(name: String, instructions: Vec<String>) -> TuringMachine {

    //variables for turing machine
    let mut state: State;
    let mut states: Vec<State> = Vec::new();
    let band: Vec<u8> = vec![0];
    let mut i: usize = 2;

    //variables for states
    let mut state_name: String = String::new();
    let mut write_zero: u8 = 0;
    let mut write_one: u8 = 0;
    let mut move_dir_zero: String = String::new();
    let mut move_dir_one: String = String::new();
    let mut next_state_zero: String = String::new();
    let mut next_state_one: String = String::new();

    //index for checking position in instruction
    let mut ind: usize;

    //iterate over states in description
    while i < instructions.len() {
        for j in i..i+STATE_LEN {
            ind = j-i;
            if (ind) <= (STATE_LEN/2) {
                if ind == 0 {
                    state_name = instructions[j].to_string();
                } else if ind == 2 {
                    write_zero = instructions[j].parse().unwrap();
                } else if ind == 3 {
                    move_dir_zero = instructions[j].to_string();
                } else if ind == 4 {
                    next_state_zero = instructions[j].to_string();
                }
            } else {
                if ind == 6 {
                    write_one = instructions[j].parse().unwrap();
                } else if ind == 7 {
                    move_dir_one = instructions[j].to_string();
                } else if ind == 8 {
                    next_state_one = instructions[j].to_string();
                }
            }
        }
        //println!("{}, {}, {}, {}", state_name, write_zero, move_dir_zero, next_state_zero);
        //println!("{}, {}, {}, {}", state_name, write_one, move_dir_one, next_state_one);
        let op_zero = Operation {write: write_zero, 
                                 move_direction: move_dir_zero.to_string(), 
                                 next_state: next_state_zero.to_string()};
        let op_one = Operation {write: write_one,
                                move_direction: move_dir_one.to_string(),
                                next_state: next_state_one.to_string()};
        state = State {name: state_name.to_string(), 
                       operation_zero: op_zero, 
                       operation_one: op_one};

        states.push(state);
        i += STATE_LEN;
    }
    
    TuringMachine {name: name.to_string(),
                   states: states,
                   band: band.to_vec()}
}

fn run_tm(mut tm: TuringMachine, start_state: String, iterations: usize) -> i64 {
    let mut state: String = start_state.to_string();
    let mut position: i64 = 0;
    let mut op: Operation;

    for _ in 0..iterations {
        for st in &tm.states {
            if st.name == state {
                if tm.band[position as usize] == 0 {
                    op = st.operation_zero.clone();
                } else {
                    op = st.operation_one.clone();
                }
                tm.band[position as usize] = op.write;
                state = op.next_state;
                if op.move_direction == "right" {
                    position += 1;
                } else {
                    position -= 1;
                }
                if position > (tm.band.len() as i64)-1 {
                    tm.band.push(0);
                } else if position < 0 {
                    tm.band.insert(0,0);
                    position = 0;
                }
                break;
            }
        }
    }
    tm.band.iter().map(|&s| s as i64).sum::<i64>()
}
