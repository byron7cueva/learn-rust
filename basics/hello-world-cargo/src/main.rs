fn main () {
  // Por defecto todas las variables en rust son definidas como inmutables
  // let x = 45;
  // Definiendo la variable como mutable
  let mut x = 45;
  
  println!("The value of x is {} ", x);

  x = 60;
  println!("The value of x is {}", x);
}