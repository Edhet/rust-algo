use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
use std::env;

extern crate num_cpus;

fn main() {

    let argument: Vec<String> = env::args().collect();
    let elapsed_t = Instant::now();

    let arg_buffer: i64 = argument[2].trim().parse().expect("Error transforming String to i64");
    let total: i64 = arg_buffer;
    let mut result: i64 = 0;

    let arg_buffer: i64 = argument[1].trim().parse().expect("Error transforming String to i64");
    let threads: i64 = arg_buffer;

    let mut processes = vec![];
    let (sech, rech) = mpsc::channel();

    let mut start: i64 = 2;
    let factor: i64 = total/threads;
    let mut end: i64 = factor;

    for i in 1..=threads {
        let sech_c = sech.clone();
  
        if i == threads && end != total {
            end = total;          
        }

        processes.push(thread::spawn(move || {
            sech_c.send(primes(start, end)).unwrap();
        }));

        start = end + 1;
        end += factor;
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

fn primes(start: i64, end: i64) -> i64 {

    let mut prime_count: i64 = 0;

    for number in start..=end{
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