const STEPSIZE: usize = 377;
const ITERATIONS_1: usize = 2017;
const ITERATIONS_2: usize = 50000000;

fn main() {
    println!("{}", create_buffer(ITERATIONS_1));
    println!("{}", part2(ITERATIONS_2));
}

fn create_buffer(iters: usize) -> i32{

    let mut position: usize = 0;
    let mut insert_size: i32 = 1;

    let mut circular_buffer: Vec<i32> = vec![0];

    for _ in 0..iters {
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

fn part2(iters: usize) -> usize {
    let mut re: usize = 0;

    let mut position: usize = 0;

    let mut buffersize: usize = 1;

    for i in 0..iters {
        position = ((position + STEPSIZE) % buffersize) + 1;
        if position == 1 {
            re = i+1;    
        }
        buffersize += 1;
    }
    re
}
