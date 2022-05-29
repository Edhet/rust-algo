use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
use std::env;

extern crate num_cpus;

pub fn call() {

    let argument: Vec<String> = env::args().collect();
    let elapsed_t = Instant::now();

    let arg_buffer: i64 = argument[2].trim().parse().expect("Error transforming String to i64");
    let total: i64 = arg_buffer;
    let mut result: i64 = 0;

    let arg_buffer: i64 = argument[1].trim().parse().expect("Error transforming String to i64");
    let threads: i64 = arg_buffer;

    let mut numbers_vector: Vec<i64> = Vec::new();
    for number in 2..=total {
        numbers_vector.push(number);
    }
    println!("{:?}", numbers_vector);

    let mut processes = vec![];
    let (sech, rech) = mpsc::channel();

    for _i in 1..=threads {
        let sech_c = sech.clone();
        let numbers_vector_c = numbers_vector.clone();
        
        let mut divided_numbers: Vec<i64> = Vec::new();
        for entry in numbers_vector_c {
            if entry % 2 != 0 || entry == 2 {
                divided_numbers.push(entry);
            }
        }
        println!("{:?}", divided_numbers);
        



        processes.push(thread::spawn(move || {
            sech_c.send(primes(divided_numbers)).unwrap();
        }));
    }

    for members in processes {
        let _ = members.join();
    }
    
    drop(sech);
    for received in rech{
        result += received;
    }

    let elapsed_t = elapsed_t.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed_t);

    println!("{} numbers where counted\n{} primes where found\nUsing {} threads.", total, result, threads);

}

fn primes(numbers: Vec<i64>) -> i64 {

    let mut prime_count: i64 = 0;

    for number in numbers {
        if is_prime(number) == true {
            prime_count += 1;
        }

    }
    return prime_count;
}

fn is_prime (value: i64) -> bool {
    
    for divisors in 2..=value/2 {
        if value % divisors == 0 {
                return false;
        }
    }
    return true;
}  