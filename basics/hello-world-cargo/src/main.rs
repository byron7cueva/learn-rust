// Estructura
struct Triangulo {
  base: f64,
  altura: f64
}

// Tratos -> Una interfaces
trait Figura {
  fn area(&self) -> f64; // Que hacer
}

// Implementacion del trait
impl Figura for Triangulo {
  fn area(&self) -> f64 { // Como hacer
    (self.base * self.altura) / 2.0
  }
}

// Implementacion -> Comportamiento
impl Triangulo {
  fn set_base(&mut self, base: f64) {
    self.base = base;
  }
}

fn main () {
  let mut triangulo = Triangulo {
    base: 10.0,
    altura: 20.0
  };

  let mut triangulo2 = Triangulo {
    base: 55.0,
    altura: 30.0
  };

  triangulo2.base *= 10.0;
  triangulo.set_base(20.0);

  println!("La base es: {}, la altura es {}", triangulo.base, triangulo.altura);
  println!("La base es: {}, la altura es {}", triangulo2.base, triangulo2.altura);
  println!("El area es: {}", triangulo.area());
}