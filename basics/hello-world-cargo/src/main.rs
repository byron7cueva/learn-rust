// Writen to a file

use std::fs::File;
use std::io::prelude::*;

fn main () {
  // Create a file
  let mut file = File::create("output.txt")
  .expect("Could not create file");

  // Write in the file
  file.write_all(b"Wellcome to Byron code")
  .expect("Cannot write to the file");
}