fn main () {
  // Por defecto todas las variables en rust son definidas como inmutables
  // let x = 45;
  // Definiendo la variable como mutable
  let mut x = 45; // i32 Numero entero de 32
  
  println!("The value of x is {} ", x);

  x = 60;
  println!("The value of x is {}", x);

  // Datat types
  // Entero
  let mut y: i64 = 45; // Definiendo entero de 64
  let a: u64 = 45; // Sin signo
  y = -45;

  // Flotante
  let f = 6.72; // f32
  let fa: f32 = 6.72; // f32
  let fb: f64 = 6.23;

  // Booleano
  let b: bool = false;
}