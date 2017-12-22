use std::io;
use std::io::{BufRead, BufReader};

const NUM_DIRECTIONS: i8 = 4;

fn main() {

    let buf_reader = BufReader::new(io::stdin());
    let input: Vec<Vec<char>> = buf_reader.lines()
                                          .flat_map(Result::ok)
                                          .map(|l| l.chars().collect())
                                          .collect();

    println!("{}", infect(input.to_vec(), 10000, false));
    println!("{}", infect(input.to_vec(), 10000000, true));
}

fn infect(input: Vec<Vec<char>>, iterations: usize, part: bool) -> i32 {

    //mutable copy of the input
    let mut grid = input.to_vec();

    //start position at middle
    let mut pos_x: i32 = (input.len()/2) as i32;
    let mut pos_y: i32 = (input.len()/2) as i32;
    
    //value between 0-3 to determine direction to move. 0 = North rest cw
    let mut heading: i8 = 0;

    // single field, infected or clean
    let mut field: char;
    
    //burst that caused infection
    let mut infection_burst: i32 = 0;

    for _ in 0..iterations {

        field = grid[pos_y as usize][pos_x as usize];

        /*for row in &grid {
            println!("{}", row.iter().collect::<String>());
        }
        println!("");*/

        if !part {
            if field == '.' {
                // turn left
                heading -= 1;
                if heading < 0 {
                    heading += NUM_DIRECTIONS;
                }
            
                //infect field
                grid[pos_y as usize][pos_x as usize] = '#';
    
                //increment burst infection
                infection_burst += 1;
    
            } else if field == '#' {
                //turn right
                heading = (heading + 1) % NUM_DIRECTIONS;
            
                //clean field
                grid[pos_y as usize][pos_x as usize] = '.';
            }
        } else {
            if field == '.' {
                //turn left
                heading -= 1;
                if heading < 0 {
                    heading += NUM_DIRECTIONS;
                }

                //weaken field
                grid[pos_y as usize][pos_x as usize] = 'W';

            } else if field == '#' {
                //turn right
                heading = (heading + 1) % NUM_DIRECTIONS;
                
                //flag field
                grid[pos_y as usize][pos_x as usize] = 'F';

            } else if field == 'W' {
                //infect node and increase count
                grid[pos_y as usize][pos_x as usize] = '#';
                infection_burst += 1;
            } else if field == 'F' {
                //reverse direction
                heading = (heading + 2) % NUM_DIRECTIONS;

                //clean node
                grid[pos_y as usize][pos_x as usize] = '.';
            }
        }

        //move in given direction
        if heading == 0 {
            pos_y -= 1;
        } else if heading == 1 {
            pos_x += 1;
        } else if heading == 2 {
            pos_y += 1;
        } else if heading == 3 {
            pos_x -= 1;
        }

        //extend grid if index is out of bounds
        if pos_x < 0 || pos_x >= grid[0].len() as i32 || pos_y < 0 || pos_y >= grid.len() as i32 {
            let dimension = grid.len();
            grid.insert(0, vec!['.';dimension+2]);
            grid.push(vec!['.';dimension+2]);
            for i in 1..grid.len()-1 {
                grid[i].insert(0,'.');
                grid[i].push('.');
            }

            //update pos_x and pos_y
            if pos_x < 0 {
                pos_x = 0;
                pos_y += 1;
            } else if pos_y < 0 {
                pos_y = 0;
                pos_x += 1;
            } else {
                pos_x += 1;
                pos_y += 1;
            }
        }

    }

    infection_burst
}
