use std::io;
use rand::Rng;

struct UserComment {
    name: String,
    comment: String,
    rating: i8,
    identifier_num: i32,
}

pub fn call() {

    let mut number_buffer = String::new();

    let mut comment = UserComment{
        name: String::new(),
        comment: String::new(),
        rating: 0,
        identifier_num: 0,
    };
    
    println!("What is your name? ");
    io::stdin()
    .read_line(&mut comment.name)
    .expect("Error reading name!");

    println!("What do you have to say about this CLI ranking system!? ");
    io::stdin()
    .read_line(&mut comment.comment)
    .expect("Error reading comment!");

    println!("How would you rank your experience from 0 to 10!?");
    io::stdin()
    .read_line(&mut number_buffer)
    .expect("Error reading numbered ranking");

    let number_buffer: i8 = number_buffer.trim().parse().expect("Error transforming String to i8");
    comment.rating = number_buffer;

    if comment.rating > 10 {
        comment.rating = 10;
    }
    else if comment.rating < 0 {
        comment.rating = 0;
    }
    else {}

    let number_buffer: i32 = rand::thread_rng().gen_range(-std::i32::MAX..=std::i32::MAX);
    comment.identifier_num = number_buffer;

    println!("\nName: {}\nComment: {}\nRating: {}\nIdentifier: {}", comment.name.trim(), comment.comment.trim(), comment.rating, comment.identifier_num);
}

