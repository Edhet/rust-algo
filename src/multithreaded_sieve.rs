use std::{
    env, process,
    sync::{mpsc::{self}, Arc, Mutex},
    time::Instant,
};

use crate::threadpool::Threadpool;

extern crate num_cpus;

const AVAILABLE_PARAMETERS: &'static [&'static str] = &["help", "threads", "max"];
const DEFAULT_MAX_NUMBER: usize = 250_000;

const SUCCESS_EXIT_CODE: i32 = 0;
const ERROR_EXIT_CODE: i32 = 1;

pub fn call() {
    let arguments: Vec<String> = env::args().collect();
    let argument_counter_without_path = arguments.len() - 1;

    let mut thread_count: usize = num_cpus::get();
    let mut max_count: usize = DEFAULT_MAX_NUMBER;
    // let mut thread_count: usize = 1;
    // let mut max_count: usize = 100;

    if argument_counter_without_path == 0 {
        println!("No arguments passed, defaulting to {max_count} in {thread_count} threads.\nType --help to see the available parameters");
    } else if argument_counter_without_path == 1 {
        if arguments[1] == "--help" {
            show_help_information();
        } else {
            invalid_argument_inputed();
        }
    } else if argument_counter_without_path % 2 != 0 {
        invalid_argument_inputed();
    } else {
        let mut parsed_arguments = vec![];
        let mut creating_new_arg = true;
        let mut argument_being_parsed = ("", 0);

        for arg in arguments[1..].iter() {
            match creating_new_arg {
                true => {
                    if !arg.starts_with("--") || !is_valid_argument(&arg) {
                        invalid_argument_inputed();
                    }
                    argument_being_parsed.0 = &arg[2..];
                    creating_new_arg = false;
                }
                false => {
                    match arg.parse::<usize>() {
                        Ok(parsed_value) => argument_being_parsed.1 = parsed_value,
                        Err(_) => invalid_argument_inputed(),
                    }

                    if (argument_being_parsed.0 == "threads" && argument_being_parsed.1 == 0)
                        || (argument_being_parsed.0 == "max" && argument_being_parsed.1 < 2)
                    {
                        invalid_argument_inputed();
                    }
                    parsed_arguments.push(argument_being_parsed);
                    creating_new_arg = true;
                    argument_being_parsed = ("", 0);
                }
            }
        }

        for argument in parsed_arguments {
            match argument.0 {
                "threads" => thread_count = argument.1,
                "max" => max_count = argument.1,
                _ => panic!("An invalid argument has being parsed"),
            }
        }
    }

    let elapsed_time = Instant::now();

    let result = multithreaded_erathostenes_sieve(thread_count, max_count);
    // let result = singlethreaded_erathostenes_sieve(max_count);

    println!("\nElapsed time: {:.2?}\n{max_count} numbers where counted\n{result} primes where found\nUsing {thread_count} threads.", elapsed_time.elapsed());
}

fn invalid_argument_inputed() {
    println!("Usage: EXECUTABLE --[PARAMETER] [VALUE]...\nType in --help to get information on the available parameters");
    process::exit(ERROR_EXIT_CODE);
}

fn show_help_information() {
    const HELP_INFORMATION: &str = r#"Multithreaded erathostenes sieve benchmark
Counts all prime numbers up to a target max number

--help      Shows help information
--threads   Number of threads to be executed on (has to be a positive number)
--max       Target max amount of numbers to calculate (has to be a positive number)
"#;
    println!("{}", HELP_INFORMATION);
    process::exit(SUCCESS_EXIT_CODE);
}

fn is_valid_argument(arg: &String) -> bool {
    for ele in AVAILABLE_PARAMETERS {
        if arg[2..] == **ele {
            return true;
        }
    }
    return false;
}

fn singlethreaded_erathostenes_sieve(max: usize) -> usize {
    let mut prime_array = vec![true; max as usize];

    let mut p: usize = 2;
    while p.pow(2) <= max - 1 {
        if prime_array[p] == true {
            let mut i = p.pow(2);
            while i <= max - 1 {
                prime_array[i] = false;
                i += p;
            }
        }
        p += 1;
    }

    return prime_array[2..]
        .iter()
        .filter(|n| **n == true)
        .collect::<Vec<_>>()
        .len();
}

fn multithreaded_erathostenes_sieve(thread_count: usize, max_count: usize) -> usize {
    let prime_array = Arc::new(Mutex::new(vec![true; max_count]));
    let pool = Threadpool::new(thread_count as i32);

    let mut p: usize = 2;
    while p.pow(2) <= max_count - 1 {
        if prime_array.lock().unwrap()[p] == true {
            let mut end = p.pow(2);

            let multiples_of_p = (max_count - end) / p;
            let multiples_per_thread = multiples_of_p / thread_count;
            let numbers_per_thread = multiples_per_thread * p;

            for _i in 0..thread_count {
                let start = end;
                end += numbers_per_thread;
                if end >= (numbers_per_thread * thread_count) {
                    end = max_count - 1;
                }

                let prime_array_copy = prime_array.clone();

                pool.execute(Arc::new(move || {
                    mark_prime_multiples(&prime_array_copy, p, start, end);
                }));
            }

            pool.join();
        }
        p += 1;
    }

    let mut prime_counter = 0;
    for is_prime in prime_array.lock().unwrap()[2..].iter() {
        if *is_prime == true {
            prime_counter += 1;
        }
    }
    return prime_counter;
}

fn mark_prime_multiples(array: &Arc<Mutex<Vec<bool>>>, prime: usize, start: usize, end: usize) {
    let mut i = start;
    while i <= end {
        array.lock().unwrap()[i] = false;
        i += prime;
    }
}
