use std::time::Instant;
use std::env;

pub fn call() -> i64 {

    let argument: Vec<String> = env::args().collect();

    let elapsed_t = Instant::now();

    let max_number: i64 = argument[1].trim().parse().expect("Error on Str -> i64");
    let mut prime_count: i64 = 0;

    for number in 2..max_number{
        if is_prime(number) == true {
            prime_count += 1;
        }
    }

    let elapsed_t = elapsed_t.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed_t);
    println!("{} numbers where counted\n {} primes where found.", max_number, prime_count);
    return prime_count;
}

fn is_prime (value: i64) -> bool {
    
    for divisors in 2..value/2 {
        if value % divisors == 0 {
                return false;
        }
    }
    return true;
}