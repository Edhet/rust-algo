pub fn call (max: i64) {
    let mut punctuation = ",";

    for number in 1..=max {
        if number == max {
            punctuation = ".";
        }
        if number % 15 == 0 {
            print!(" Fizz Buzz{punctuation}");
        }
        else if number % 5 == 0 {
            print!(" Buzz{punctuation}");
        }
        else if number % 3 == 0 {
            print!(" Fizz{punctuation}");
        }
        else {
            print!(" {number}{punctuation}");
        }
    }
    print!("\n");
}