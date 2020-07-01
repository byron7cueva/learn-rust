// Enumerados
// Enums permite definir un tipo enumerando sus posibles valores
// Definiremos y usaremos un enum para mostrar cómo se puede codificar el significado junto con los datos
/* enum Payment {
  // Las siguientes se las conoce como variantes del enum
  Cash,
  CreditCard,
  DebitCard,
}

fn main () {
  // las variantes de la enumeración están en el namespace de su identificador y usamos los ::
  let some_payment = Payment::Cash;

  match some_payment {
    Payment::Cash => {
      println!("Paying with cash...");
    }
    Payment::CreditCard => {
      println!("Paying with credit card...");
    }
    /* Payment::DebitCard => {
      println!("Paying with debit card...");
    }*/
    _ => {}
  }
}*/

// Los enums tienen un descriminador implicito. Las variantes toman valores empezando desde 0
enum Number {
  Zero,
  One,
  Two
}

// Enum con descriminador explicito, es decir indicando su valores a cada variante
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff
}

enum Payment {
  // Definiendo valores asociados a las variantes
  Cash(f32), // Value
  CreditCard(String, f32), // Tuple
  DebitCard(DebitData), // Struct
  Crypto{account_id: String, amount: f32},
}

struct DebitData {
  pub card_number: String,
  pub amount: f32
}

// Este código muestra que se puede poner cualquier tipo de datos dentro de una variante de enum:
// cadenas, tipos numéricos o estructuras. Incluso puedes incluir otra enumeración.
struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr {
  v4(Ipv4Addr),
  v6(Ipv6Addr)
}

// Definir un enum con variantes como estas es similar a definir diferentes tipos de estructuras,
// excepto que el enum y todas las variantes se agrupan bajo el un tipo único
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

// También es posible implementar métodos sobre los enums
impl Message {
  fn call(&self) {
    println!("Call");
  }
}

fn main () {
  // Los enums pueden ser casteados a enteros
  println!("Zero es {}", Number::Zero as i32);
  println!("One es {}", Number::One as i32);
  println!("Two es {}", Number::Two as i32);

  println!("rojo es #{:06x}", Color::Red as i32);
  println!("azul es #{:06x}", Color::Blue as i32);

  // Adjuntamos datos a cada variante de la enum directamente, por lo que no hay necesidad de una estructura extra
  let some_payment = Payment::Cash(100.);
  proccess_payment(some_payment);

  let cc_payment = Payment::CreditCard("CC Num".to_string(), 250.);
  proccess_payment(cc_payment);

  let debit_payment = Payment::DebitCard(DebitData {
    card_number: "Debit num".to_string(),
    amount: 100.,
  });
  proccess_payment(debit_payment);

  let crypto_payment = Payment::Crypto{account_id: "abc 123".to_string(), amount: 20.};
  proccess_payment(crypto_payment);
}

fn proccess_payment(some_payment: Payment) {
  match some_payment {
    Payment::Cash(amt) => {
      println!("Paying with cash... in the amount of {}", amt);
    }
    /* Payment::CreditCard(dsc, amt) => {
      println!("Paying with credit card... Desc is {} and amount is {} ", dsc, amt);
    } */
    /* Payment::CreditCard(dsc, _amt) => { //Poniendole _amt no valida si se esta utilizando el parametro
      println!("Paying with credit card... Desc is {}", dsc);
    } */
    Payment::CreditCard(dsc, _) => { //Poniendole _ no valida que le hace falta otro parametro de usar
      println!("Paying with credit card... Desc is {}", dsc);
    }
    Payment::DebitCard(data) => {
      println!("Paying with debit card... card_number {}, amount {}", data.card_number, data.amount);
    }
    Payment::Crypto{account_id, amount} => {
      println!("Paying with crypto... account_id {}, amount {}", account_id, amount);
    }
  }
}