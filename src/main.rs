use mymath::maths::square;
use stringmul::stringmul::multiply_string;
use std::io::Write;
use std::str::FromStr;

/// prints a sum of squares of the command line arguments
fn main() {
    let numbers = vec_from_args();
    validate_numbers(&numbers);
    let d = sum_vector(&numbers);
    println!("Sum of numbers is {}!", d);
    let x = multiply_string(String::from("bar"), d);
    println!("{}", x)
}

/// Generates a sum of squares of the vector of integers
fn sum_vector(numbers: &Vec<u64>) -> u64 {
    let mut d = square(numbers[0]);
    for m in &numbers[1..] {
        d += square(*m);
    }
    d
}

/// Ensure that we get at least one number
fn validate_numbers(numbers: &Vec<u64>) {
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: sum NUMBER ...").unwrap();
        std::process::exit(1);
    }
}

/// Generate a vector of integers from the command line arguments
fn vec_from_args() -> Vec<u64> {
    let mut numbers = Vec::new();
    for arg in std::env::args().skip(1) {
        let parse_result = u64::from_str(&arg);
        match parse_result {
            Err(e) => {
                writeln!(std::io::stderr(), "*** {} generated an error {:?}", arg, e).unwrap();
            }
            Ok(v) => {
                numbers.push(v);
            }
        }
    }
    numbers
}
