const STEPSIZE: usize = 377;
const ITERATIONS: i32 = 2017;

fn main() {
    println!("{}", create_buffer());
}

fn create_buffer() -> i32{

    let mut position: usize = 0;
    let mut insert_size: i32 = 1;

    let mut circular_buffer: Vec<i32> = vec![0];

    for _ in 0..ITERATIONS {
        position = (position + STEPSIZE) % circular_buffer.len();
        if position + 1 == circular_buffer.len() {
            circular_buffer.push(insert_size);
        } else {
            circular_buffer.insert(position + 1, insert_size);
        }
        insert_size += 1;
        position += 1;
    }

    circular_buffer[(position+1)%circular_buffer.len()]
}
