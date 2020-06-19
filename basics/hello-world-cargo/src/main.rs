// Random
use rand::{thread_rng, Rng};

fn main () {
  let mut rng = thread_rng();
  let random_number: u32 = rng.gen_range(1, 11); // Numero randon entre 1-10
  println!("The random number is: {}", random_number);

  let random_bool: bool = rng.gen_bool(1.0 / 3.0);
  println!("Random boolean is {}", random_bool);
}