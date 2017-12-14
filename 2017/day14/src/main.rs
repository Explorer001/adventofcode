extern crate hashes;
extern crate regex;

use hashes::knothash::knothash;
use regex::Regex;

fn main() {
    let input = "wenycdww".to_string();
    println!("{}", get_used_field_count(input));
}

fn get_used_field_count(hash: String) -> i32 {
    let mut re: i32 = 0;
    let mut hs = hash.to_string();
    let orig_hash = hash.to_string();
    let mut hash_bin: String;
    for i in 0..128 {
        hs.push('-');
        hs.push_str(&(i.to_string()));
        hash_bin = get_binary(knothash::hash(hs.to_string()));
        re += hash_bin.chars().map(|c| c.to_string().parse::<i32>().unwrap()).sum::<i32>();
        hs = orig_hash.to_string();
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
