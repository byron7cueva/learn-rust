// Read file

use std::fs::File;
use std::io::prelude::*;

fn main () {
  // Open the file
  let mut file = File::open("info.txt").expect("Can't open the file");

  // Create new string variable
  let mut contents = String::new();

  // Read the file
  file.read_to_string(&mut contents)
    .expect("Can not read the file");

  // Print the content of file
  println!("File contents:\n\n{}", contents);

}