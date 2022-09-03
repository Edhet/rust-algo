use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
use std::io;
use std::env;

extern crate num_cpus;

const START: i64 = 3;

pub fn call() -> io::Result<i64> {

    let argument: Vec<String> = env::args().collect();
    let elapsed_t = Instant::now();

    let threads;
    let max_number;

    if argument.len() == 3 {
        let arg_buffer = argument[1].trim().parse().unwrap_or(num_cpus::get());
        if arg_buffer <= num_cpus::get() * 3 {
            threads = arg_buffer;
        }
        else {
            println!("Can only spawn 3x the CPUs threads, defaulted to actual number of Threads.");
            threads = num_cpus::get();
        }

        let arg_buffer = argument[2].trim().parse().unwrap_or(250000);
        if arg_buffer > 3 {
        max_number = arg_buffer;
        }
        else {
            println!("Cannot count numbers up to below 3, defaulted to 250k.");
            max_number = 250000;
        }
    }
    else {
        println!("Wrong arguments, defaulting to number of Threads in CPU and 250k.");
        threads = num_cpus::get();
        max_number = 250000;
    }

    let divided_lists = divide_numbers(max_number, threads);
    let result = do_thread_work(divided_lists, threads);

    let elapsed_t = elapsed_t.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed_t);
    println!("{max_number} numbers where counted\n{result} primes where found\nUsing {threads} threads.");
    
    return Ok(result);
}

fn divide_numbers(max_number: i64, threads: usize) -> Vec<Vec<i64>> {
    let mut list  = vec![];

    for _thread in 1..=threads {
        let buffer_vec = vec![];
        list.push(buffer_vec);
    }

    let mut thread_index = 0;
    for value in START..=max_number {
        if value % 2 != 0 {
            list[thread_index].push(value);

            thread_index += 1;
            if thread_index == threads {
                thread_index = 0;
            }
        }
    } 
    list[0].push(2);

    return list;
}

fn do_thread_work (list: Vec<Vec<i64>>, threads: usize) -> i64 {
    let mut result = 0;

    let mut processes = vec![];
    let (sech, rech) = mpsc::channel();

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