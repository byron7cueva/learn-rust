// Comand line arguments
// Run with: cargo run arg1 arg2

use std::env;

fn main () {
  let args: Vec<String> = env::args().collect();

  for argument in args {
    println!("{}", argument);
  }
}