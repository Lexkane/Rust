use std::io;
 fn get_console_integer() -> u32 { println!("Enter a whole number: ");

let mut input = String::new();
 io::stdin().read_line(&mut input).expect("Did not enter a correct string");
  let input: u32 = input.trim().parse().unwrap(); input }

fn main() { println!("This program will generate fibonacci series upto n-th number"); 
let number =get_console_integer();
let mut counter = 0; while counter < number { print!("{} ", fibonacci(counter));
 counter = counter + 1; }
println!(); }

fn fibonacci(number: u32) -> u32 { if number == 0 { 0
} else if number == 1 { 1} else { fibonacci(number -1) + fibonacci(number -2) } }