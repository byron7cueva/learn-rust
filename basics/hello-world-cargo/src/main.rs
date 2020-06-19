// Enum methods
#![allow(dead_code)] // Para ignorar opciones del enum que no se esten utilizando

enum Day {
  Monday,
  Tuesday,
  Wednesday,
  Thursday,
  Friday,
  Saturday,
  Sunday
}

impl Day {
  fn is_weekday(&self) -> bool {
    match self {
      &Day::Saturday | &Day::Sunday => return false,
      _ => return true
    }
  }
}

fn main () {
  let d = Day::Saturday;

  println!("Is day weekday: {}", d.is_weekday());
}