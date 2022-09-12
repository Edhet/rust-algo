use std::io;
use std::time::Instant;

const MAX_ITER: u16 = 184;

pub fn progression() -> io::Result<()>{

    println!("How many times should fibonacci iterate?");
    println!("Max iterations: {}\n", MAX_ITER);

    let inputed_iterator;

    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim().parse::<u16>();
        match user_input {
            Ok(user_input) => {
                if user_input <= MAX_ITER {
                    inputed_iterator = user_input;
                    break    
                }
                else {
                    println!("Too much iterations, would cause overflow.");
                }
            },
            Err(e) => println!("Wrong input, {e}.")
        }
    }

    println!("\n");
    let elapsed_time = Instant::now();
    {
        let mut value_x: u128 = 1;
        let mut value_y: u128 = 0;
        let mut value_z: u128;
        for i in 0..=inputed_iterator {
            value_z = value_x + value_y;
            value_y = value_x;
            value_x = value_z;
            println!("{}, {}", value_y, i);
        }
    }
    let elapsed_time = elapsed_time.elapsed();
    println!("Elapsed Time: {:.2?}", elapsed_time);
    Ok(())
}