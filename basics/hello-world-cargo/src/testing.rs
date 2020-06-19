// Test

struct Rectangle {
  width: u8,
  height: u8
}

impl Rectangle {
  fn is_square(&self) -> bool {
    self.width == self.height
  }
}

fn main () {

}

fn give_two() -> i32 {
  2
}

#[cfg(test)]
mod tests {
  #[test] // Indicar una funcion test
  #[should_panic] // Indicando que puede generar error
  fn test_basic () {
    assert!(1 == 1); // Ok
    panic!("Oh no"); // Fail the test
  }

  #[test]
  // #[ignore] // Para ignorar test
  fn test_equals() {
    assert_eq!(2, 1 + 1);
    assert_ne!(2, 1 + 2);
    assert_eq!(super::give_two(), 1 + 1 );
  }


  #[test]
  #[should_panic] // Para dejar pasar los errores
  fn test_structs() {
    let r = super::Rectangle {
      width: 50,
      height: 25
    };

    assert!(r.is_square());
  }
}