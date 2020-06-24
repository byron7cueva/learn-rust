// Casting, Shadowing, constants and statics
#[allow(unused_variables)]

// Constants
const DOUGS_CONSTANT: i64 = 1000;

// Statics
static mut MY_STATIC_VARIABLE: i32 = 10;

fn main () {
  let some_i32: i32 = 10;
  let some_f64: f64 = 20.2;

  // Casting
  let combined = some_i32 + some_f64 as i32;
  println!("{}", combined);

  // Shadowing
  let var_a: i32 = 10;

  {
    println!("Dentro de un scope interno se puede ver fuera del scope de var_a {}", var_a);
    let var_a: f32 = 20.222; // Se crea un variable totalmente diferente en el scope interno, no hace referencia a la que 
    // se creo fuera de este scope, no afeact al scope de afuera
    println!("Se puede sombrear con otro propietario dentro de este scope {}", var_a);
  }

  println!("La variable no fue afectada {}", var_a);

  // Constants
  println!("Constant is {}", DOUGS_CONSTANT);
  let circle_pi = std::f32::consts::PI;
  println!("{}", circle_pi);

  // Statics
  // Se recomienda utilizar constantes
  unsafe {
    MY_STATIC_VARIABLE = 20;
    println!("{}", MY_STATIC_VARIABLE);
  }
}