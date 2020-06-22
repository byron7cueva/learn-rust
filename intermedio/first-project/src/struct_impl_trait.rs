mod random_info;
use random_info::*;

// struct, trait, impl

#[allow(dead_code)]
#[derive(Debug, Clone)] // Implementando macros
struct DougsData {
  some_int: i32,
  some_float: f64,
  some_bool: bool,
  random: RandomInfo
}

impl RandomInfo {
  pub fn is_larger(&self, compare_to: i64) -> bool {
    self.some_int > compare_to
  }
}

// Polimorfismo
impl SomeTrait for DougsData {
  fn is_valid(&self) -> bool {
    true
  }
}

fn print_if_is_valid(check_me: &dyn SomeTrait) {
  if check_me.is_valid() {
    println!("Yay!");
  }
}

// Default
impl Default for DougsData {
  fn default() -> Self {
    Self {
      some_bool: true,
      some_float: 10.3,
      some_int: 80,
      random: RandomInfo::new(true),
    }
  }
}

#[allow(unused_variables)]
fn main () {
  let mut dougs_var: DougsData = DougsData {
    some_bool: true,
    some_float: 10.3,
    some_int: 80,
    random: RandomInfo {
      call_count: 0,
      some_bool: true,
      some_int: 10
    }
  };

  dougs_var.some_int = 100;

  let dougs_var_2 = DougsData {
    some_int: 200,
    random: RandomInfo::new(false),
    ..dougs_var
  };

  let dougs_var_3 = DougsData::default();
  // #[derive(Debug)] Se debe agregar a la estructura
  println!("{:?}", dougs_var_3);


  let mut random_info_var = RandomInfo {
    call_count: 0,
    some_bool: true,
    some_int: 10
  };

  let is_this_smaller = random_info_var.is_smaller(9);
  let is_this_large = random_info_var.is_larger(20);
  let is_valid = random_info_var.is_valid();

  print_if_is_valid(&random_info_var);
  print_if_is_valid(&dougs_var);
}