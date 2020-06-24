// Print and format macros
#[allow(unused_variables)]

#[derive(Debug)] // Procedual Macro, se los utiliza como decoradores
struct DougsData {
  pub a: i32,
  pub b: f32
}

fn main () {
  // print
  print!("Hello");
  println!(" Patner");

  let more_data = 6.7;
  println!("My data is {} and {}", 5, more_data);
  println!("My data is {1} and {0}", 5, more_data); // Cambiando el orden de la impresion de los parametros
  println!("My name is {first_name} {last_name}", first_name = "Byron", last_name = "Cueva");

  let data = DougsData {
    a: 1,
    b: 1.1
  };

  let other_data = DougsData {
    a: 2,
    b: 2.2
  };

  println!("Doug's data is {:?}", data);
  println!("Doug's data is {:#?}", data);
  println!("Doug's data is {:#?} and {:#?}", data, other_data);
  println!("Doug's data is {1:#?} and {0:#?}", data, other_data);

  // format! -> Declarative Macro
  let some_formatted_string = format!("Doug's data is {1:#?} and {0:#?}", data, other_data);
  println!("{}", some_formatted_string);
}