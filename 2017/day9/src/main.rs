use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: String = buf_reader.lines()
                                  .flat_map(Result::ok)
                                  .map(|s| s.to_string())
                                  .collect();
    //println!("{}", input);
    println!("{:?}", get_score(input.to_string()));
}

fn get_score(input: String) -> (i32, i32) {
    let mut score: i32 = 0;
    let mut garbage_count: i32 = 0;
    let mut group_score: i32 = 0;
    let mut is_garbage: bool = false;
    let mut skip_next: bool = false;
    for c in input.chars() {
        if skip_next {
            skip_next = false;
            continue;
        } else if c == '{' && !is_garbage {
            group_score += 1;
        } else if c == '}' && !is_garbage {
            score += group_score;
            group_score -= 1;
        } else if c == '<' && !is_garbage {
            is_garbage = true;
        } else if c == '>' {
            is_garbage  = false;
        } else if c == '!' && is_garbage {
            skip_next = true;
        } else if is_garbage {
            garbage_count += 1;
        }
    }
    (score, garbage_count)
}
