// Structs

struct Color {
  red: u8, // u8: 0 - 255
  green: u8,
  blue: u8
}

fn main () {
  // Create a object of type Color
  // let bg = Color { red: 255, green: 70, blue: 20 };
  let mut bg = Color { red: 255, green: 70, blue: 20 };
  let fc: Color = Color { red: 255, green: 70, blue: 20 };

  bg.red = 15;
  println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}