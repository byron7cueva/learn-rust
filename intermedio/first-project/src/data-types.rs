#[allow(unused_variables)] // I don't have warnings about unused vars
#[allow(unused_assignments)] // I don't have warnings about unused assignments

fn main () {
  // The convention is all lowercase separate with _
  // The varianles by default is inmutable
  // let some_data: bool = true; // Boolean data type
  // some_data = false; // Esto da un error ya qu no es mutable

  let some_number: i8 = 10; // from -128 to +127
  println!("Min i8 is {}", std::i8::MIN);
  println!("Max i8 is {}", std::i8::MAX);

  // let dougs_test = some_number + 120; // 130, witch is greater than the max 128
  // println!("{}", dougs_test); // This will panic, which means crash

  /* let some_number2: u8 = 10; // from 0 to 255
  let some_other_number = 255 + some_number2; // Error too big
  let some_other_number2 = 1 - some_number2; // Error Too small */

  // Integer (+/-) i8, i16, i32, i64, i128
  // Unsigned u8, u16, u32, u64, u128
  // Architecture Dependent: isize and usize Se utiliza para la indexacion de arrays
  let some_number3: i128 = 10; //
  println!("Min i128 is: {}", std::i128::MIN);
  println!("Max i128 is: {}", std::i128::MAX);

  // Por defecto se infiere en i32, si este se le pasa como parametros inferirar al tipo de dato de la funcion
  let some_number = 10;
  // Se puede utilizar giones bajos para separar numeros que son muy grandes con el fin de que sean mas faciles de leer
  let num1 = 1_000_000;
  // Se puede utilizar sufijos y esto va inferir el tipo de dato
  let num2 = 2i8;
  // Los numeros tambien tienen funciones asociadas
  println!("{}", 4i32.abs()); // Hay que indicarle el tipo de dato por eso el sufijo i32

  let some_isize: isize = 10; // dependes on wheter your computer is 32 bits or 64 bits
  let some_uisize: usize = 10; // same here

  // Tipos de datos flotantes f32, f64
  let some_number4: f32 = 10.; // Don't forget the dot somewhere in the number
  let some_number5 = 10.; // By default ser f64

  // Charts 4 bits
  // Los caracteres son de tipo unicode
  // Los Strings no son colecciones de caracteres. Los String son Buffers, colecciones de bytes de utf8
  let some_char: char = 'a';

  // Overflow
  let num3: u8 = 255;
  // Lo siguiente da un error ya que u8 solo es hasta 255, esto en modeo debug da error en tiempor de compilacion
  // pero en modeo release da 0 en tiempo de compilación, ya que cualquier cosa que se haga con enteros que lleve
  // a un overflow, se va realizar de forma rapida, que es envolverlo
  // println!("{}", a + 1); // Error de panico
  // println!("{:?}", num3.checked_add(1)); // Devuelve Some(valor) donde valor es el resultado o None en el caso que se produsca un overflow
  println!("{:?}", num3.wrapping_add(1)); // Devuelve el resultado de la suma o el equivalente de volver a empezar (u8 = 0 a 255  si se suma 2 devolveria 1)
  // Se recomienda ser explicito y utilizar por lo menos wrapping_add
  println!("{:?}", a.saturation_add(1)); // Nos va dar el mas cercano o el tope del tipo de dato si fuese el caso

  // Conversion
  // En rust la conversion inplicita no existe
  let a: u8 = 12;
  let b: i32 = 2;
  // println!("{:?}", a + b); // Esto genera un error ya que no son del mismo tipo, igual no se hace una conversion automatica
  println!("{:?}", a + b as u8);

  // Byte literals (Literales de Bytes)
  let a = b'A'; // Esto permite es el caracter poner como si fuera un byte. La A en codigo ACII es el 65
  assert_eq!(a, 65u8);

  // Booleanos
  let a: bool = false;
  assert_eq!(a, 1); // Esto no funciona en Rust, ya que se debe realizar una conversión explicita. Aca el 0 no es false ni el 1 true
  assert_eq!(a as i32, 0); // Esto da true
  // No se puede convertir de integer a booleando en este caso: a as bool -> Genera un error
}