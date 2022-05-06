use std::io;
use std::time::Instant;

pub fn progression() {

    println!("How many times should fibonacci iterate?");
    println!("Max iterations: {}\n", std::u16::MAX);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error_1");
    let mut user_input: u16 = user_input.trim().parse().expect("Error_2");
    println!("\n");
    
    let elapsed_time = Instant::now();
    {
        let mut value_x: u128 = 1;
        let mut value_y: u128 = 0;
        let mut value_z: u128 = 0;

        loop {
            if user_input != 0 {
                value_z = value_x + value_y;
                value_y = value_x;
                value_x = value_z;
                user_input = user_input - 1;
                println!("{}", value_y);
            }
            else {
                break;
            }
        }
    }
    let elapsed_time = elapsed_time.elapsed();
    println!("Elapsed Time: {:.2?}", elapsed_time);
}