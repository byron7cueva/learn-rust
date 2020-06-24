// Con el signo de admiracion !, ejemplo: println!(), estas son macros
// Las macros permiten definir un lenguaje a un dominio especifico

#[derive(Debug)]
struct DougsStruct {
  a: i32,
  b: f64,
  c: String
}

fn main () {

  let entero: u8 = 5;
  // Imprimir en formato del apuntador
  println!("La direccion del entero es {:p}", &entero);

  // STACk
  // - Memoria rapida en crear y en recuperar.
  // - La memoria es automaticamente liberada por el programa despues que las variables salen del scope
  // - Es por defecto en rust
  // - Variables de tamaño fijo. las colecciones no son basadas en stack, los Strings son un coleccion de u8
  let stack_i8: i8 = 10;
  let stack_f32: f32 = 10.0;
  let stack_bool: bool = true;
  let stack_char: char = 'a';

  // HEAP
  // - Es flexible
  // - La memoria puede crecer en tamaño (Vector, HashMap, String, etc)
  // - Tiene un costo en el performance
  // - Esta memoria puede vivir mas alla que el scope en donde se la creo
  // - La memoria es liberada automaticamente cuando el ultimo propietario sale del scope
  let heap_vectos: Vec<i8> = Vec::new();
  let heap_string: String = String::from("Hello");
  let heap_i8: Box<i8> = Box::new(30);


  // Las copias de stack no tiene mucho costo
  let stack_i8_2 = stack_i8; // stack_i8 y stack_i8_2 son propietarios con diferente espacio de memoria
  println!("{}", stack_i8);
  println!("{}", stack_i8_2);

  // En Rust un espacio de memoria tiene un dueño
  // Solo puede haber un único propietario de la memoria a la vez
  let heap_i8_2 = heap_i8; // El propietario del espacio de memoria es movido a heap_i8_2
  // println!("{}", heap_i8); // Error heap_i8 ya no es propietario del espacio en memoria
  println!("{}", heap_i8_2);

  // Borrow (&)
  // Prestamo como referencia
  let heap_i8_3: Box<i8> = Box::new(20);
  // Se puede clonar pero esto es costoso para HEAP
  // let heap_i8_4 = heap_i8_3.clone(); // Crea una copia de memoria
  let heap_i8_4 = &heap_i8_3;

  //
  let stack_f64: f64 = 1.;
  let heap_f64: Box<f64> = Box::new(2.);
  let heap_f64_2: Box<f64> = Box::new(2.);
  let mut heap_f64_3: Box<f64> = Box::new(3.);
  let heap_f64_4: Box<f64> = Box::new(4.);

  stack_procedure(stack_f64); // stack_f64 es copiado a param de la funcion
  println!("In main stack {}", stack_f64); // No afecta a la variable lo que hizo en la funcion

  heap_procedure(heap_f64); // Se transfiere la propiedad de la memoria de heap:f64 a param
  //println!("In main heap {}", heap_f64); // Error ya que se movio la propiedad del espacio de momoria

  heap_procedure(heap_f64_2.clone()); // Tiene el mismo efecto que se produce con Stack, pero tiene mayor costo
  // ya que se esta creando otro espacio de memoria
  println!("In main heap {}", heap_f64_2);

  // Se puede obtener nuevamente la referencia
  // Pero esto no es recomendable si se aumentan los parametros
  heap_f64_3 = heap_procedure2(heap_f64_3);
  println!("In main heap {}", heap_f64_3);

  // Borrow
  // Solo un propietario de memoria a la vez
  heap_procedure3(&heap_f64_4); // Pasadolo como una referencia prestada
  println!("In main heap {}", heap_f64_4);


  let some_string: String = String::from("Hello"); // Los estrings siempre estan en un heap
  let some_str: &str = "Partner"; // &str Es un puntero ya sea para stack o heap
  // let some_str3: str = "Partner"; // No se puede realizar esto ya que str siempre debe ser un apuntador &str
  some_procedure(some_string, some_str);
  // println!("{} {}", some_string, some_str); // Error ya que some_string es String y es un Heap y se movio la propiedad de memoria a la funcion

  let some_string2: String = String::from("Hello");
  let some_str2: &str = "Pather";
  some_procedure2(&some_string2, some_str2);
  println!("{} {}", some_string2, some_str2);


  let var_a = String::from("Hello");
  let var_b = &var_a;
  let var_c = &var_a;
  println!("{} {} {}", var_a, var_b, var_c);


  let mut var_1 = DougsStruct { a: 9, b: 10., c: "a".to_string()};
  struct_procedure(&mut var_1);
  println!("{:?}", var_1);
}

/* fn stack_procedure(param: f64) {
  println!("In stack_procedure with param {}", param);
} */

// param tiene diferente localizacion de memoria por lo cual no afecta a la variable que se paso
fn stack_procedure(mut param: f64) {
  param += 9.;
  println!("In stack_procedure with param {}", param);
}

// Al llamar a la funcion se pasa la propiedad del espacio de memoria a param
// Este espacio de memoria se limpia al salir del de la funcion
fn heap_procedure(param: Box<f64>) {
  println!("Inheap_procedure with param {}", param);
}

fn heap_procedure2(param: Box<f64>) -> Box<f64>{
  println!("Inheap_procedure with param {}", param);
  param
}

// Borrow (&)
fn heap_procedure3(param: &Box<f64>) {
  println!("In heap_procedure with param {}", param);
}

fn some_procedure(param_a: String, param_b: &str) {
  println!("{} {}", param_a, param_b);
}

fn some_procedure2(param_a: &String, param_b: &str) {
  println!("{} {}", param_a, param_b);
}

fn struct_procedure(param: &mut DougsStruct) {
  param.a = 15;
  println!("{:?}", param);
}