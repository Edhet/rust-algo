use std::io;

pub fn calc(){

    let mut memory: f64 = 0.0;

    println!("Welcome to CALC-CLI-RUST 1.0v\n");

    loop {
        println!("Select an operation:\n1: Addition\n2: Subtraction\n3: Multiplication\n4: Division\n9: Exit\n");
        println!("\nCurrent value: {}\n", memory);

        let mut user_operation = String::new();
        io::stdin()
        .read_line(&mut user_operation)
        .expect("Error on Input");
        let mut user_operation: u8 = match user_operation.trim().parse() {
            Ok(num) => (num),
            Err(_) => continue,
        };

        if user_operation == 9 {
            break;
        }

        else {
            println!("Input number:");
            let mut user_number = String::new();
            io::stdin()
            .read_line(&mut user_number)
            .expect("Error on Input");
            let mut user_number: f64 = match user_number.trim().parse() {
                Ok(num) => (num),
                Err(_) => continue,
            };

            match user_operation {
                1 => {
                    memory = memory + user_number;
                    println!("Result: {}\n", memory);
                }
                2 => {
                    memory = memory - user_number;
                    println!("Result: {}\n", memory);
                }
                3 => {
                    memory = memory * user_number;
                    println!("Result: {}\n", memory);
                }
                4 => {
                    memory = memory / user_number;
                    println!("Result: {}\n", memory);
                }
                _ => {
                    println!("Given operation is not specified");
                }
            }
        }

    }
}