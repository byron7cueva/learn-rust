fn main () {
  print_numbers_to(10);
}

fn print_numbers_to (num: u32) { // This function receibe a number unsign of 32 bits
  for n in 1..num {
    if is_even(n) {
      println!("{} is even", n);
    } else {
      println!("{} is odd", n);
    }
  }
}

fn is_even (num: u32) -> bool { // This function return to boolean
  return num % 2 == 0;
}  