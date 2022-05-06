use std::io;

pub fn check(){
   loop {
        let mut user_input = String::new();
        io::stdin()
        .read_line(&mut user_input)
        .expect("error1");
        let user_input: i32 = match user_input.trim().parse(){
            Ok (num) => (num),
            Err(_) => break,
        };
        
        for n in 1..=user_input {
            if user_input % n == 0 {
                let value = n;
                if value == 1 & user_input {
                    println!("is Prime");
                    break;
                }
                else {
                    println!("is not Prime");
                    break;
                }
            }
            else {
                continue;
            }
        }
    }
}