use std::env;

pub fn call() -> i32 {
    let args: Vec<String> = env::args().collect();
    let mut int_args: Vec<i32> = Vec::new();
    
    let mut common_div: i32 = 0; 

    let mut entry: usize = 1;
    while entry < args.len() {
        int_args.push(args[entry].trim().parse().expect("Error pushing or converting."));
        entry += 1;
    }

    println!("The GCD of {} and {} is: ", int_args[0], int_args[1]);
    if int_args[0] == 0 {
        common_div = int_args[1];
    }
    else if int_args[1] == 0 {
        common_div = int_args[0];
    }
    else {
        for divisor in 1..=int_args[0] {
            if int_args[0] % divisor == 0 && int_args[1] % divisor == 0 {
                common_div = divisor;
            }
        }
    }
    println!("{}", common_div);
    return common_div;
}