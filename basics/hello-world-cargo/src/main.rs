// References
// COn el simbolo & se realiza una referencia a una variable

fn main () {
  let mut x = 10;

  // Creando una referencia
  // No puede afectar el valor
  //let xr = & x;
  // Referencia mutable
  // let xr = &mut x;
  // *xr = 11;
  //let dom = & x;
  // println!("The value to x is {}", x); // Da error

  {
    let xr = &mut x;
    *xr = 11;
    println!("The value to x is {}", xr);
  }
  println!("The value to x is {}", x);
}