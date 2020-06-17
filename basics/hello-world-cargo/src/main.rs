// Shadowing

fn main () {
  let mut x = 10;

  {
    // Si se define una variable con el mismo nombre de otra que esta fuera del bloque, se define otra y no afecta al entorno global
    let x = 15;
  }

  println!("The x value is {}", x);

  let x = "X is string";
  println!("The x value is {}", x);
  
  let x = true;
  println!("The x value is {}", x);
}