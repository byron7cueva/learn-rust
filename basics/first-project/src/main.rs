// Tuples
#[allow(unused_variables)]

fn main () {
  // Las tupas pueden contener tuplas
  let some_tuple = (2, 3.4, "a", "b".to_string(), 'c', (1.1, 2.2));
  println!("My data is  {} {}", some_tuple.0, some_tuple.1);
  println!("My full tuple is {:?}", some_tuple);
  let some_val = (some_tuple.5).1;

  let some_color = get_some_rgb();
  println!("Green is {}", some_color.1);

  let (my_red, my_green, my_blue) = some_color;
  println!("r {}, g {}, b {}", my_red, my_green, my_blue);

  // rgb or argb?
  let some_other_color: (u8, u8, u8, u8) = (0, 100, 150, 255);
  // Empty tuple
  let empty_tuple = ();

  match some_color.2 {
    0..=200 => println!("blah"),
    _ => ()
  }
}

// Funcion que retorna una tupla
fn get_some_rgb() -> (u8, u8, u8) {
  (255, 20, 30)
}