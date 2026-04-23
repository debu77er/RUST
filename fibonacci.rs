// rustc fibonacci.rs
// ./fibonacci 10

use std::env;

fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument was provided
    if args.len() != 2 {
        eprintln!("Usage: {} <number_of_terms>", args[0]);
        return;
    }

    // Parse the argument to an integer
    let n: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a valid positive integer");
            return;
        }
    };

    // Generate and print Fibonacci sequence
    let mut a: u64 = 0;
    let mut b: u64 = 1;

    for i in 0..n {
        if i == 0 {
            println!("{}", a);
        } else if i == 1 {
            println!("{}", b);
        } else {
            let next = a + b;
            println!("{}", next);
            a = b;
            b = next;
        }
    }
}