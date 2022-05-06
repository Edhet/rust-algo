use std::time::Instant;

pub fn call(max: i64) -> i64 {

    let elapsed_t = Instant::now();
    
    let max_number: i64 = max;
    let mut prime_count: i64 = 0;
    {
        for number in 2..=max_number {
            if number % 2 != 0 || number == 2 {
                for divisor in 2..=number {
                    if divisor % 2 != 0 && number % divisor == 0 || number == 2 {
                        if divisor == number {
                            prime_count += 1;
                        }
                        else  {
                            break;
                        }
                    }
                    else {
                        continue;
                    }
                }
            }
            else {
                continue;
            }
        }
        println!("{} numbers where counted\n {} primes where found.", max_number, prime_count);
    }
    let elapsed_t = elapsed_t.elapsed();
    println!("Elapsed time: {:.2?}", elapsed_t);

    return prime_count;
}