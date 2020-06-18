// Implements

struct Rectangle {
  width: i32,
  height: i32
}

// Permite implementar metodos a una estructura

impl Rectangle {

  // self hace referencia a la instancia de la estructura
  fn print_description (&self) {
    println!("Rectangle {} x {}", self.width, self.height);
  }

  fn area(&self) -> i32 {
    (self.height * self.height) / 2
  }

  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn main () {
  let my_rec = Rectangle {
    width: 10,
    height: 5
  };

  my_rec.print_description();
  println!("The area is: {}", my_rec.area());
  println!("The retangle is square {}", my_rec.is_square());
}