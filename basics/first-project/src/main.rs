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
  // Architecture Dependent: isize and usize
  let some_number3: i128 = 10; //
  println!("Min i128 is: {}", std::i128::MIN);
  println!("Max i128 is: {}", std::i128::MAX);

  let some_number = 10; // By default set i32

  let some_isize: isize = 10; // dependes on wheter your computer is 32 bits or 64 bits
  let some_uisize: usize = 10; // same here

  // Floating f32, f64
  let some_number4: f32 = 10.; // Don't forget the dot somewhere in the number
  let some_number5 = 10.; // By default ser f64

  // Charts 4 bits
  let some_char: char = 'a';
}