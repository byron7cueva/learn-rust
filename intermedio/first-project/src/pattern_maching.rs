#[derive(Debug)] // Es una anotacion para imprimir el objeto
struct Ejemplo {
  a: i32,
  b: i32
}

fn main () {
  // Todas las variables por defecto son inmutables, no podemos modificar su valor
  let x: i32 = 1;
  println!("x = {}", x);

  // x = 2; // No se puede porque la variable no es mutable
  let mut y: i32 = 1;
  y = 2;
  println!("x = {}", y);

  // Rust puede inferir el tipo de dato
  let z = 3;

  // Constantes
  // En las constantes es requiro simpre indicar el tipo de dato
  // Se las puede declarar a nivel global fuera del main y se puede tener acceso a ella en cualquier lugar
  // No se puede asignar a una constante un valor devuelto por una funcion MI_CONST = funcion();
  // Se debe asignar algo que no es dinamico
  const MI_CONST: i32 = 1;
  println!("La constante es: {}", MI_CONST);

  // Tupla es un agragado que puede ser del mismo tipo o de diferente tipo
  let some_data = (1,2,"hola");

  // Pattern Maching
  // let PATRON = EXPRESION
  let (a1, a2, a3) = (1,2,3);
  println!("a1 = {}, a2 = {}, a3 = {}", a1, a2, a3);

  let ejemplo = Ejemplo { a: 10, b: 20 };
  println!("ejemplo = {:?}", ejemplo);

  // Desestructurando
  // let Ejemplo {a, b} = Ejemplo { a: 10, b: 20 };
  // println!("a = {}, b = {}", a, b);
  let Ejemplo {a: c, b: d} = Ejemplo { a: 10, b: 20 };
  println!("a = {}, b = {}", c, d);
}