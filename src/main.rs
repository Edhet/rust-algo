use std::{thread, vec};
use std::time::Instant;
use std::sync::mpsc;
extern crate num_cpus;

fn main() {
    let elapsed_t = Instant::now();

    let total: i64 = 2500;
    let start: i64 = 2;
    let end: i64 = total/2;
    let mut result: i64 = 0;

    let threads = num_cpus::get() as i64;

    let mut processes = vec![];
    let (sech, rech) = mpsc::channel();

    for _i in 0..threads {
        let sech_c = sech.clone();

        processes.push(thread::spawn(move || {
            sech_c.send(primes(start, end)).unwrap();
        }));
    }

    for members in processes {
        let _ = members.join();
    }
    drop(sech);
    
    for received in rech{
        result += received;
    }
    println!("{}", result);


/*
    let mut processes = vec![];
    let (send_channel, receive_channel) = mpsc::channel();
    let mut result: i64 = 0;
    
    for _i in 0..threads {
        processes.push(thread::spawn(||{
            send_channel.send(primes(start1, end1)).unwrap();  
        }));
    }
    
    for received in receive_channel{
        println!("{} numbers where counted\n {:?} primes where found.", total, received);
    }
    
    
    for members in processes {
        let _ = members.join();
    } 

    
    let (send_channel_1, receive_channel_1) = mpsc::channel();
    let (send_channel_2, receive_channel_2) = mpsc::channel();

    let start_1: i64 = 2;
    let end_1: i64 = total/2;

    let start_2: i64 = end_1+1;
    let end_2: i64 = total;

    let mut result: i64 = 0;

    let thread_1 = thread::spawn(move || {
    send_channel_1.send(primes(start_1, end_1)).unwrap();   
    });

    let thread_2 = thread::spawn(move || {
    send_channel_2.send(primes(start_2, end_2)).unwrap();
    });

    result += receive_channel_1.recv().unwrap();
    result += receive_channel_2.recv().unwrap();

    thread_1.join().unwrap();
    thread_2.join().unwrap();
*/
    let elapsed_t = elapsed_t.elapsed();
    println!("\nElapsed time: {:.2?}", elapsed_t);

    //println!("{} numbers where counted\n {} primes where found.", total, result);
    
}

fn primes(start: i64, end: i64) -> i64 {

    let mut prime_count: i64 = 0;

    for number in start..end{
        if number % 2 != 0 || number == 2{
            if is_prime(number) == true {
                prime_count += 1;
            }
        }
    }
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