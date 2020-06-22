pub trait SomeTrait {
  fn is_valid(&self) -> bool;
  // fn get_the_better_one(&self, some_other: &Self) -> Self;
}


// Para poder utilizar la estructura desde otro archivo se debe indicar que es publica
#[derive(Debug, Clone)]
pub struct RandomInfo {
  // Para poder utilzar las propiedades desde de otro archivo se debe indicar que es publico las propiedades
  pub call_count: i64,
  pub some_bool: bool,
  pub some_int: i64
}

impl SomeTrait for RandomInfo {
  fn is_valid(&self) -> bool {
    self.some_bool
  }
}

impl RandomInfo {
  // El metodo new es una convencion para crear una instancia
  // Cuando no recibe como parametros a self este se llamara con ::
  pub fn new(param_a: bool) -> Self {
    Self {
      call_count: 0,
      some_bool: !param_a,
      some_int: 8,
    }
  }

  // Recibe como primer parametro self que es la referecia
  // Cuando recibe el parametro &self, este metodo se llamara con .
  // pub fn is_smaller(&self, compate_to: i64) -> bool {
  // Se debe indicar que es mutable para que se pueda modificar el valor
  // Al indicar que es mutable la variable de el tipo de la estructura para llamar a este metodo tambien se
  // debe indicar que es mutable
  pub fn is_smaller(&mut self, compate_to: i64) -> bool {
    self.call_count += 1;
    self.some_int < compate_to
  }
}

struct LookMaNoFields {}

// Generics
struct Pair<T> { x: T, y: T, }

// Tuple
struct Color(u8, u8, u8);