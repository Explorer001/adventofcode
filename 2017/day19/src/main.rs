use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<String>> = buf_reader.lines()
                                            .flat_map(Result::ok)
                                            .map(|l| l.chars()
                                                      .map(|e| e.to_string())
                                                      .collect())
                                            .collect();

    println!("{}", follow(input.to_vec()));
}

fn follow(input: Vec<Vec<String>>) -> String {
    let mut re = String::new();
    
    let mut move_x: i16 = 0;
    let mut move_y: i16 = 1;
    
    let mut pos_x: usize = input[0].iter().position(|e| e == "|").unwrap();
    let mut pos_y: usize = 0;

    let mut old_x: usize = 0;
    let mut old_y: usize = 0;

    let mut element: String;

    let mut reached_end = false;

    let mut steps: i32 = 0;

    while !reached_end {
        
        element = input[pos_y][pos_x].to_string();
        
        if element == " " {
            reached_end = true;
        } else if element == "+" {
            let adjs = get_adjacent(pos_y, pos_x, input.len()-1, input[pos_y].len()-1);
            for adj in &adjs {
                let (y,x) = *adj;
                if input[y][x] != " " && x != old_x && y != old_y {
                    move_x = x as i16 - pos_x as i16; 
                    move_y = y as i16 - pos_y as i16;
                    break;
                }
            }
        } else if element != "|" && element != "-" {
            re.push_str(&element);
        }

        old_x = pos_x;
        old_y = pos_y;

        pos_x = ((pos_x as i16) + move_x) as usize;
        pos_y = ((pos_y as i16) + move_y) as usize;
        
        steps += 1;
    }

    println!("Steps: {}", steps-1);

    re
}

fn get_adjacent(y: usize, x: usize, y_max: usize, x_max: usize) -> Vec<(usize,usize)> {
    let mut re: Vec<(usize,usize)> = Vec::new();
    if y == 0 {
        if x == 0 {
            re.push((y+1,x));
            re.push((y,x+1));
        } else if x == x_max {
            re.push((y,x-1));
            re.push((y+1,x));
        } else {
            re.push((y+1,x));
            re.push((y,x-1));
            re.push((y,x+1));
        }
    } else if y == y_max {
        if x == 0 {
            re.push((y-1,x));
            re.push((y,x+1));
        } else if x == x_max {
            re.push((y,x-1));
            re.push((y-1,x));
        } else {
            re.push((y-1,x));
            re.push((y,x-1));
            re.push((y,x+1));
        }
    } else {
        if x == 0 {
            re.push((y-1,x));
            re.push((y+1,x));
            re.push((y,x+1));
        } else if x == x_max {
            re.push((y-1,x));
            re.push((y,x-1));
            re.push((y+1,x));
        } else {
            re.push((y-1,x));
            re.push((y+1,x));
            re.push((y,x-1));
            re.push((y,x+1));
        }
    }
    re
}
