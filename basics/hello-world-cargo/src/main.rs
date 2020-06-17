// Infinite loop
fn main () {
  let mut n = 0;

  // This is a loop
  loop {
    n += 1;

    if n == 7 {
      // skip to next iteration
      continue;
    }

    if n > 10 {
      // Finalice loop
      break;
    }

    println!("The n is {}", n);
  }
}