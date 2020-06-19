// Execute comands
use std::process::Command;

fn main () {
  // python util.py
  // Se debe pasar el nombre del comando a ejecutar
  let mut cmd = Command::new("python");
  cmd.arg("util.py");

  //Execute comand
  match cmd.output() {
    Ok(o) => {
      unsafe {
        println!("Output: {}", String::from_utf8_unchecked(o.stdout));
      }
    },
    Err(e) => {
      println!("There was an error: {}", e);
    }
  }
}