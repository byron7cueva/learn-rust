// Modules
mod modulo {
  pub fn print_message () {
    message();
    println!("How's it going");
  }

  fn message () {
    println!("This is message");
  }

  pub mod water {
    pub fn print_message_water () {
      println!("I'm water");
    }
  }
}

fn main () {
  modulo::print_message();
  modulo::water::print_message_water();
}