use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<i32>> = buf_reader.lines()
                                         .flat_map(Result::ok)
                                         .map(|s| s.split(',')
                                                   .map(|s| s.to_string()
                                                             .parse()
                                                             .unwrap())
                                                   .collect())
                                        .collect();
    let mut hashlist: Vec<i32> = Vec::new();
    for i in 0..256 {
        hashlist.push(i);
    }
    println!("{}", hash(&mut hashlist, input[0].to_vec()));
}

fn hash(hashlist: &mut Vec<i32>, length_vector: Vec<i32>) -> i32{
    let mut position: i32 = 0;
    let mut skip_size: i32 = 0;
    let mut temp: i32;
    let mut end: i32;
    let mut temp_begin: i32;
    let mut temp_end: i32;
    let mut i: i32;
    let hash_len: i32 = hashlist.len() as i32;
    for element in &length_vector {
        end = (position + *element - 1)%hash_len;
        println!("Indexing from {} to {}", position, end);
        i = 0;
        while i < (*element-1)/2 {
            temp_begin = (position + i)%hash_len;
            if end - i < 0 {
                temp_end = ((end - i)%hash_len) + hash_len;
            } else {
                temp_end = end - i
            }
            temp = hashlist[temp_begin as usize];
            hashlist[temp_begin as usize] = hashlist[temp_end as usize];
            hashlist[temp_end as usize] = temp;
            i += 1;
        }
        position = (position + *element + skip_size)%hash_len;
        skip_size += 1;
    }
    hashlist[0]*hashlist[1]
}
