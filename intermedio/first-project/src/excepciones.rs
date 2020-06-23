//use std;
use std::io::{stdin}; //Traendo solo stdin

fn main () {
  let mut text = String::new();
  // Capturando entrada del usuario
  // std::io::stdin()
  /* stdin().read_line(&mut text) //Este devuelve un Result el cual es un enumerado
    .expect("Error"); // Si hay un error se ejecuta esta funcion */


  // Atacando a la enumeracion de forma directa
  match stdin().read_line(&mut text) {
    Ok(valor) => {
      println!("{}", valor); // Cantidad de bytes que se recibe a traves del read line
      println!("{}", text);
    },
    Err(error) => println!("EL error es {}", error)
  }
  
  println!("{}", text);
}