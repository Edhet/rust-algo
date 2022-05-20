use std::time::Instant;

pub fn call(max: i64) -> i64 {
    let elapsed_t = Instant::now();
    
    let max_number: i64 = max;
    let mut prime_count: i64 = 0;

    for number in 2..max_number{
        if number % 2 != 0 || number == 2{
            if is_prime(number) == true {
                prime_count += 1;
            }
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