// Hash Map

use std::collections::HashMap;

fn main () {
  let mut marks = HashMap::new();

  // Add values
  marks.insert("Rust programing", 7);
  marks.insert("Web Development", 9);
  marks.insert("UX Designer", 8);
  marks.insert("UI Designer", 10);

  // Find lenght of hasmap
  println!(" How many subject have you studied? {}", marks.len());

  // Get a single value
  match marks.get("Web Development") {
    Some(mark) => println!("You got {} for Web Dev", mark),
    None => println!("You didn't study  Web Development")
  }

  // Remove a value
  marks.remove("UX Designer");

  // Loop through HashMap
  for (key, value) in &marks {
    println!("For {} you got {}", key, value);
  }

  // Check for value
  println!("Did you study C++? {}", marks.contains_key("C++ Programing"));
}