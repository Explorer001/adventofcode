pub fn hash(in_str: String) -> String{
    let range: i32 = 64;
    let mut position: i32 = 0;
    let mut skip_size: i32 = 0;
    let mut temp: i32;
    let mut end: i32;
    let mut temp_begin: i32;
    let mut temp_end: i32;

    let mut length_vector: Vec<i32> = in_str.chars()
                                            .map(|c| (c as u8) as i32)
                                            .collect();

    let extension: Vec<i32> = vec![17, 31, 73, 47, 23];
    length_vector.extend(extension.iter().cloned());

    let mut hashlist: Vec<i32> = Vec::new();
    for i in 0..256 {hashlist.push(i)};
    let hash_len = hashlist.len() as i32;

    for _ in 0..range {
        for element in &length_vector {
            //println!("{}", *element);
            if (position + *element -1)%hash_len < 0 {
                end = ((position + *element -1)%hash_len) + hash_len;
            } else {
                end = (position + *element - 1)%hash_len;
            }
            //println!("Indexing form {} to {}", position, end);
            for i in 0..(*element)/2 {
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
    
    let mut hash_digest: String = String::new();
    let mut part_vec: Vec<i32> = Vec::new();

    for i in 0..16 {
        part_vec.clear();
        for j in 16*i..16*(i+1) {
            part_vec.push(hashlist[j]);
        }
        hash_digest.push_str(&xor_part(part_vec.to_vec()));
    }

    hash_digest
}

fn xor_part(part: Vec<i32>) -> String {
    let mut re: i32 = part[0];
    for i in 1..part.len() {
        re ^= part[i];
    }
    //println!("{}", re);
    format!("{:01$x}", re, 2)
}
