// Reading user input

use std::io;

fn main () {
  let mut input = String::new();

  println!("Hi! Say something");

  match io::stdin().read_line(&mut input) {
    // Cuando el ingreso es correcto
    Ok(_) => {
      // println!("Success, You said: {}", input);
      println!("Success, You said: {}", input.to_uppercase());
    },
    // Cuando ocurre un error en el ingreso de datos
    Err(e) => {
      println!("Oops! Something went wrong {}", e);
    }
  }
}