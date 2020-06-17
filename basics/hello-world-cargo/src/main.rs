// While estructure control
// Also use the break and continue into the while

fn main () {
  let mut n = 0;

  while n <= 50 {

    // Print if n is multiple for 5
    if n%5 == 0 {
      println!("The value of n is {}", n);
    }

    n += 1;
  }
}