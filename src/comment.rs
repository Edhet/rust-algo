use std::io;
use rand::Rng;

pub fn call() {

    let mut number_buffer = String::new();
    let mut comment: (i8, String, String, i32) = (0, String::new(), String::new(), 0);
    
    println!("What is your name? ");
    io::stdin()
    .read_line(&mut comment.1)
    .expect("Error reading name!");

    println!("What do you have to say about this shit-ass CLI ranking!? ");
    io::stdin()
    .read_line(&mut comment.2)
    .expect("Error reading comment!");

    println!("How would you rank your experience FROM -{} TO {}!?", std::i8::MAX, std::i8::MAX);
    io::stdin()
    .read_line(&mut number_buffer)
    .expect("Error reading numbered ranking");

    let mut number_buffer: i8 = number_buffer.trim().parse().expect("Error transforming String to i8");
    comment.0 = number_buffer;

    let mut number_buffer: i32 = rand::thread_rng().gen_range(-std::i32::MAX..=std::i32::MAX);
    comment.3 = number_buffer;

    println!("\nNumerical ranking: {};\nUsername: {};\nComment: {};\nUnique identifier: {}.", comment.0, comment.1.trim(), comment.2.trim(), comment.3);
}

