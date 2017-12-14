extern crate hashes;
extern crate regex;

use hashes::knothash::knothash;
use regex::Regex;

fn main() {
    let input = "wenycdww".to_string();
    //let test = "flqrgnkx".to_string();
    println!("{:?}", get_frag_info(input));
}

fn get_frag_info(hash: String) -> (i32, i32) {
    let mut re: i32 = 0;
    let mut hs = hash.to_string();
    let orig_hash = hash.to_string();
    let mut hash_bin: String;

    let mut grid: Vec<Vec<i32>> = Vec::new();

    for i in 0..128 {
        hs.push('-');
        hs.push_str(&(i.to_string()));
        hash_bin = get_binary(knothash::hash(hs.to_string()));
        grid.push(hash_bin.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect());
        re += hash_bin.chars().map(|c| c.to_string().parse::<i32>().unwrap()).sum::<i32>();
        hs = orig_hash.to_string();
    }
    
    let mut objectives: Vec<(usize,usize)> = Vec::new();

    let x_max = grid[0].len()-1;
    let y_max = grid.len()-1;
    let mut removed_group: bool;
    let mut group_count = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            removed_group = false;
            if grid[i][j] != 0 {
                removed_group = true;
                grid[i][j] = 0;
                objectives.extend(get_adjacent(i,j,y_max, x_max).iter());
                while !objectives.is_empty() {
                    let (y,x) = objectives.pop().unwrap();
                    if grid[y][x] != 0 {
                        grid[y][x] = 0;
                        objectives.extend(get_adjacent(y,x,y_max, x_max).iter());
                    }
                }
            }
            if removed_group {
                group_count += 1;
            }
        }
    }

    (re, group_count)
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

fn get_binary(lit: String) -> String {
    let reg = Regex::new(r"[0-9]").unwrap();
    let mut re: String = String::new();
    for c in lit.chars() {
        let number: bool = reg.is_match(&c.to_string());
        match number {
            true => re.push_str(&format!("{:04b}", c.to_string().parse::<i32>().unwrap())),
            false => re.push_str(&format!("{:04b}", format_hex(c.to_string()))),
        }
        
    }
    re
}

fn format_hex(lit: String) -> i32 {
    let re: i32;
    match lit.as_ref() {
        "a" => re = 10,
        "b" => re = 11,
        "c" => re = 12,
        "d" => re = 13,
        "e" => re = 14,
        "f" => re = 15,
        _ => re = 1337,
    }
    re
}
