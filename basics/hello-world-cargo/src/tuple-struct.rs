// Tuple Struct

struct Color(u8, u8, u8);

fn main () {
  // let red = Color(255, 0, 0);
  let mut red = Color(255, 0, 0);
  red.2 = 20;

  println!("The color is {}, {}, {}", red.0, red.1, red.2);
}