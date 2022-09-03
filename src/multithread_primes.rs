use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
use std::io;
use std::env;

extern crate num_cpus;

const START: u64 = 3;

pub fn call() -> io::Result<i64>{
    let arguments: Vec<String> = env::args().collect();
    let elapsed_t = Instant::now();

    let threads;
    let max_number;

    if arguments.len() == 1 {
        threads = num_cpus::get();
        max_number = 250000;
        println!("No argument given, defaulting to {threads} threads up to {max_number}.");
    }
    else if arguments.len() == 3 {
        let thread_arg = arguments[1].trim().parse();
        match thread_arg {
            Ok(thread_arg) => {
                if thread_arg <= num_cpus::get() * 3 {
                    threads = thread_arg
                }
                else {
                    panic!("Thread spawn limited to three times your CPU threads.")
                }
            },
            Err(_e) => panic!("Thread argument not positive integer.")
        }

        let number_arg = arguments[2].trim().parse();
        match number_arg {
            Ok(number_arg) => {
                if number_arg > 3 {
                    max_number = number_arg
                }
                else {
                    panic!("Number has to be bigger than 3.")
                }
            },
            Err(_e) => panic!("Number argument not positive integer.")
        }
    }
    else {
        panic!("Wrong number of arguments.");
    }

    let divided_lists = divide_numbers(max_number, threads);
    let result = do_thread_work(divided_lists, threads);

    let elapsed_t = elapsed_t.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed_t);
    println!("{max_number} numbers where counted\n{result} primes where found\nUsing {threads} threads.");
    
    return Ok(result);
}

fn divide_numbers(max_number: u64, threads: usize) -> Vec<Vec<u64>> {
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

fn do_thread_work (list: Vec<Vec<u64>>, threads: usize) -> i64 {
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

fn primes(list: Vec<u64>) -> i64 {

    let mut prime_count: i64 = 0;

    for number in list{
        if is_prime(number) == true {
            prime_count += 1;
        }

    }
    return prime_count;
}

fn is_prime (value: u64) -> bool {
    
    for divisors in 2..=value/2 {
        if value % divisors == 0 {
                return false;
        }
    }
    return true;
}  