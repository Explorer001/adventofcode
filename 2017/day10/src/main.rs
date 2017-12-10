use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let in_str: String = buf_reader.lines().flat_map(Result::ok)
                                           .map(|s| s.to_string())
                                           .collect();

    let input: Vec<i32> = in_str.split(",")
                               .map(|s| s.parse()
                                         .unwrap())
                               .collect();

    let mut ascii_in: Vec<i32> = in_str.chars()
                                       .map(|c| (c as u8) as i32)
                                       .collect();

    let extension: Vec<i32> = vec![17, 31, 73, 47, 23];

    ascii_in.extend(extension.iter().cloned());

    println!("{:?}", ascii_in);

    println!("{}", hash(input.to_vec(), 1));
}

fn hash(length_vector: Vec<i32>, rounds: i32) -> i32{
    let mut position: i32 = 0;
    let mut skip_size: i32 = 0;
    let mut temp: i32;
    let mut end: i32;
    let mut temp_begin: i32;
    let mut temp_end: i32;

    let mut hashlist: Vec<i32> = Vec::new();
    for i in 0..256 {hashlist.push(i)};
    let hash_len = hashlist.len() as i32;

    for _ in 0..rounds {
        for element in &length_vector {
            end = (position + *element - 1)%hash_len;
            for i in 0..(*element-1)/2 {
                temp_begin = (position + i)%hash_len;
                if end - i < 0 {
                    temp_end = ((end - i)%hash_len) + hash_len;
                } else {
                    temp_end = end - i
                }
                temp = hashlist[temp_begin as usize];
                hashlist[temp_begin as usize] = hashlist[temp_end as usize];
                hashlist[temp_end as usize] = temp;
            }
            position = (position + *element + skip_size)%hash_len;
            skip_size += 1;
        }
    }

    hashlist[0] * hashlist[1]
}
