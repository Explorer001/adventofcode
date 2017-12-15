fn main() {
    println!("Matches: {}", solve(512, 191, 40000000));
}

fn solve(seed_a: i64, seed_b: i64, gen_range: usize) -> i32{
    let mut matches: i32 = 0;
    
    let gen_a_factor: i64 = 16807;
    let gen_b_factor: i64 = 48271;

    let divider: i64 = 2147483647;

    let mut val_a = seed_a;
    let mut val_b = seed_b;

    let mut bin_a: String;
    let mut bin_b: String;

    for _ in 0..gen_range {
        val_a = (val_a * gen_a_factor) % divider;
        val_b = (val_b * gen_b_factor) % divider;
        bin_a = get_lowest_16_binary_bits(val_a);
        bin_b = get_lowest_16_binary_bits(val_b);
        if bin_a == bin_b {
            matches += 1;
        }
    }
    matches
}

fn get_lowest_16_binary_bits(n: i64) -> String {
    let binary_string: String = format!("{:016b}", n);
    (&binary_string[(binary_string.len()-16)..binary_string.len()]).to_string()
}
