fn main() {
    let (ring, seq_min, side_len) = get_ring_info(277678);
    let (x,y) = get_position((ring, seq_min, side_len), 277678);
    let dist = manhattan_dist((0, 0), (x, y));
    let test = stress_test(side_len, 277678);
    println!("{}, {}", dist, test);
}

fn stress_test(side_len: i32, n: i32) -> i32 {
    let offset: i32 = (side_len / 2) + 1;
    let mut x = offset;
    let mut y = offset;
    let mut max_val: i32 = 0;
    let mut rs: i32 = 2;
    let mut count: i32 = 1;
    let mut dir: i32;
    let mut mat = vec![vec![0i32; (side_len + 1) as usize]; (side_len + 1) as usize];
    mat[x as usize][y as usize] = 1;
    //println!("({}, {})", x-offset, y-offset);
    x += 1;
    while max_val < n {
        //println!("({}, {})", x-offset, y-offset);
        max_val = mat[(x+1) as usize][(y+1) as usize] + mat[x as usize][(y+1) as usize] + mat[(x-1) as usize][(y+1) as usize] + mat[(x+1) as usize][(y-1) as usize] + mat[x as usize][(y-1) as usize] + mat[(x-1) as usize][(y-1) as usize] + mat[(x-1) as usize][y as usize] + mat[(x+1) as usize][y as usize];
        //println!("{}", max_val);
        mat[x as usize][y as usize] = max_val;
        dir = count/rs;
        if dir > 3 {
            rs += 2;
            count = 1;
            x += 1;
            continue;
        }
        if dir == 0 {
            y += 1;
        }
        else if dir == 1 {
            x -= 1;
        }
        else if dir == 2 {
            y -= 1;
        } else {
            x += 1;
        }
        count += 1;
    }
    return max_val;
}

fn manhattan_dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    return (x1-x2).abs() + (y1-y2).abs();
}

fn get_position(ring_info: (i32, i32, i32), n: i32) -> (i32, i32) {
    let (ring, ring_min, side_len) = ring_info;
    let mut x: i32 = ring;
    let mut y: i32 = 0-ring;
    let mut start: i32 = ring_min;
    let mut count: i32 = 0;
    let side: i32 = side_len-1;
    let mut direction: i32;
    //println!("{}, {}", x, y);
    while start <= n {
        direction = count/side;
        if direction == 0 {
            y += 1;
        }
        else if direction == 1 {
            x -= 1;
        }
        else if direction == 2 {
            y -= 1;
        }
        else {
            x += 1;
        }
        //println!("{}, {}", x, y);
        start += 1;
        count += 1;
    }
    return (x,y);
}

fn get_ring_info(n: i32) -> (i32, i32, i32) {
    let mut ring: i32 = 0;
    let mut seq_max: i32 = 1;
    let mut seq_min: i32 = 0;
    let mut side_len: i32 = 1;
    while seq_max < n {
        ring += 1;
        side_len += 2;
        seq_min = seq_max+1;
        seq_max += 8+(ring-1)*8;
    }
    return (ring, seq_min, side_len);
}
