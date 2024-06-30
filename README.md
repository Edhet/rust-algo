# Rust Algorithms

This repository is made to keep track (and backup) my progress studying data structures and algorithms in the Rust language.

### How to compile those projects?

To compile, first you need to have the rust compiler installed, then you can clone this repo and create a main.rs on the 'src' folder, import the project you want to compile and call it's public function, like the following.

    mod project_name;

    fn main() {
    project_name::public_function();
    }

Then you can just compile it by running cargo on the folder:

    cargo run -r

## multithreaded_sieve.rs

A multithreaded implementation of the erathostenes sieve (incredibly slower than the singlethreaded one).

## multithreaded_primes.rs

Finds all the prime numbers ranging from 0 to number given by argument input with multithread.

## binsearch.rs

Binary search iteratively.

## quicksort.rs

One pivot quicksort algorithm.

## bubblesort.rs

Recursive Bubble sort algorithm.

## fibonacci.rs

Simple fibonacci progression.

## fizz_buzz.rs

Fizz buzz algorithm.

## gcd.rs

Algorithms that finds the greatest common divisor between two numbers.

## primes.rs

Same as multithreaded_primes.rs but single threaded.
