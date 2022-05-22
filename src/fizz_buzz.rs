pub fn call (max: i64) {

    let max_number = max;

    for number in 1..=max_number {
        if number % 15 == 0 {
            print!("Fizz Buzz ");
        }
        else if number % 5 == 0 {
            print!("Buzz ");
        }
        else if number % 3 == 0 {
            print!("Fizz ");
        }
        else {
            print!("{} ", number);
        }
    }
    println!("");
}