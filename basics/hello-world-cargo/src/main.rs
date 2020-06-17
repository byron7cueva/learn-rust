// Enum

enum Direction {
  Up,
  Dow,
  Left,
  Right
}

fn main () {
  let player_direction: Direction = Direction::Dow; // Con :: se accede a una variable

  // Similar to switch, validate value and execute case
  match player_direction {
    Direction::Dow => println!("The direction is down"),
    Direction::Up => println!("The direction is up"),
    Direction::Left => println!("The direction is left"),
    Direction::Right => println!("The direction is right")
  }
}