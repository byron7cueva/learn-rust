// Code blocks

fn main () {
  let x = 10;

  {
    // Isolate
    // Aqui se puede acceder a variables declaradas fuera de este bloque y dentro de este
    // Las variables declaradas dentro de este bloque no se puede acceder de este

    let y = 20;
    println!("x: {}, y: {}", x, y);
  }
  // println!("x: {}, y: {}", x, y); // No se puede utilizar y desde aca
}