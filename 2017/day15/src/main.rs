fn main() {
    println!("Matches: {}", solve(512, 191, 40000000, false));
}

fn solve(seed_a: i64, seed_b: i64, gen_range: usize, part1: bool) -> i32{
    let mut matches: i32 = 0;
    
    let gen_a_factor: i64 = 16807;
    let gen_b_factor: i64 = 48271;

    let criteria_a: i64 = 4;
    let criteria_b: i64 = 8;

    let divider: i64 = 2147483647;

    let mut val_a = seed_a;
    let mut val_b = seed_b;

    let mut bin_a: String;
    let mut bin_b: String;

    let mut matching: i32 = 0;

    if part1 {
        for _ in 0..gen_range {
            val_a = (val_a * gen_a_factor) % divider;
            val_b = (val_b * gen_b_factor) % divider;
            bin_a = get_lowest_16_binary_bits(val_a);
            bin_b = get_lowest_16_binary_bits(val_b);
            if bin_a == bin_b {
                matches += 1;
            }
        }
    } else {
        let mut ok_a: bool;
        let mut ok_b: bool;
        while matching < 5000000 {
            ok_a = false;
            ok_b = false;
            while !ok_a || !ok_b {
                if !ok_a {
                    val_a = (val_a * gen_a_factor) % divider;
                    if val_a % criteria_a == 0 {
                        ok_a = true;
                    }
                }
                if !ok_b {
                    val_b = (val_b * gen_b_factor) % divider;
                    if val_b % criteria_b == 0 {
                        ok_b = true;
                    }
                }
            }
            matching += 1;
            bin_a = get_lowest_16_binary_bits(val_a);
            bin_b = get_lowest_16_binary_bits(val_b);
            if bin_a == bin_b {
                matches += 1;
            }
        }
    }

    matches
}

fn get_lowest_16_binary_bits(n: i64) -> String {
    let binary_string: String = format!("{:016b}", n);
    (&binary_string[(binary_string.len()-16)..binary_string.len()]).to_string()
}
