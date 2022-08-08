use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
use std::env;

extern crate num_cpus;

pub fn call() -> i64 {

    let argument: Vec<String> = env::args().collect();
    let elapsed_t = Instant::now();

    let arg_buffer: usize = argument[1].trim().parse().expect("Error transforming String to i64");
    let threads: usize = arg_buffer;

    let arg_buffer: i64 = argument[2].trim().parse().expect("Error transforming String to i64");
    let max_number: i64 = arg_buffer;
    let mut result: i64 = 0;

    let mut processes = vec![];
    let (sech, rech) = mpsc::channel();

    let start: i64 = 2;
    let mut list  = vec![];

    for _thread in 1..=threads {
        let buffer_vec = vec![];
        list.push(buffer_vec);
    }

    let mut thread_index = 0;
    for value in start..=max_number {
        if value % 2 != 0 {
            list[thread_index].push(value);

            thread_index += 1;
            if thread_index == threads {
                thread_index = 0;
            }
        }
    } 
    list[0].push(2);

    for i in 0..=threads - 1 {
        let sech_c = sech.clone();
        let buffer_list = list[i].clone();

        processes.push(thread::spawn(move || {
            sech_c.send(primes(buffer_list)).unwrap();
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

    println!("{} numbers where counted\n{} primes where found\nUsing {} threads.", max_number, result, threads);
    return result;
}

fn primes(list: Vec<i64>) -> i64 {

    let mut prime_count: i64 = 0;

    for number in list{
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