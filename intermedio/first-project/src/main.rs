// Lifetimes

const SOME_CONST_A: &str = "I'm a constant";
const SOME_CONST_B: &str = "I'm a constant, too";

struct DougsStruct<'a, 'b: 'a> { // 'b: a Indicando que el lifetime de b es el mismo que de a
  some_data: Vec<i32>,
  some_reference_data: &'a Vec<i32>,
  some_reference_data2: &'b Vec<i32>
}

fn main () { 
  /* let a;

  {
    let b = String::from("Hello");
    a = &b; // a no puede tomar prestada una referencia de b, ya que la vida de b solo depende de este scope
  }
  println!("{}", a); */

  let some_int_var = 10;
  let resul_ref = get_int_ref2(&some_int_var); // Rust valida que se encuentre en el mismo scope la variable a la que se devuelve la referencia
  println!("{}", resul_ref);

  //
  let a: &str = "a";
  // let greater = some_func2(a, SOME_CONST_B);
  let greater = some_func2(SOME_CONST_A, SOME_CONST_B);
  println!("{}", greater);

  //
  let some_float_1  = 1.1;
  let some_float_2: f64 = 2.2;
  let result_float = get_smaller(&some_float_1, &some_float_2);

  let some_str_1  = "a";
  let some_str_2 = "b";
  let result_str = get_smaller(&some_str_1, &some_str_2);
}

// No se puede retornar un referencia ya que el propietaario de la misma vive en el scope de la funcion
/* fn get_int_ref () -> &i32 {
  let a = 1;
  &a
} */

// Si puede retornar la referencia ya que no depende del scope de la funcion, si no que se la pasa como parametro
fn get_int_ref2(param_1: &i32) -> &i32 {
  param_1
}

fn get_int_ref3<'a>(param_1: &'a i32) -> &'a i32 {
  param_1
}

// La funcion retorna una referencia prestada pero no sabe cual es el parametro prestado a retornan si param_1 o de param_2
/* fn get_str_ref(param_1: &str, param_2: &str) -> &str {
  if param_1 > param_2 {
    param_1
  } else {
    param_2
  }
}*/

fn get_str_ref<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
  if param_1 > param_2 {
    param_1
  } else {
    param_2
  }
}

#[allow(dead_code)]
fn test_1(param_1: Vec<f64>) -> Vec<f64> { // Lifetimes no aplica ya que no tiene referencias de entrada y de salida
  param_1
}

#[allow(dead_code)]
fn test_2(param_1: &Vec<f64>) -> Vec<f64> { // Lifetimes no genera problemas ya que no se retorna una referencia
  param_1.clone()
}

#[allow(dead_code)]
fn test_3<'a>(param_1: &'a Vec<f64>) -> Vec<f64> { // Lifetimes no genera problemas ya que no se retorna una referencia
  param_1.clone()
}

/* #[allow(dead_code)]
fn test_4(param_1: Vec<f64>) -> &Vec<f64> { // Genera un error ya que se genera la referencia del parametro que es el owner
  &param_1
} */

/* #[allow(dead_code)]
fn test_5<'a>(param_1: Vec<f64>) -> &'a Vec<f64> { // Genera un error ya que se genera la referencia pero le pertenece al scope de la funcion
  &param_1 // Lifetime no se aplica ya que no es una referencia de entrada
} */

fn some_func() -> &'static str {
  SOME_CONST_A
}

fn some_func2(param_1: &'static str, param_2: &'static str) -> &'static str {
  if param_1 > param_2 {
    param_1
  } else {
    param_2
  }
}

// Genericos
fn get_smaller<'a, T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T {
  if param_1 < param_2 {
    param_1
  } else {
    param_2
  }
}