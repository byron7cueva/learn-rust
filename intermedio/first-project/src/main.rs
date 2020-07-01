// match
// Es un operador de control de flujo
// Permite comparar un valor frente a una serie de patrones y luego ejecutar código basado en el patrón
// coincidente. Los patrones pueden estar compuestos de valores literales, nombres de variables,
// carácteres, y muchas otras cosas;

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState)
}

fn main () {
  value_in_cents(Coin::Penny);
  value_in_cents(Coin::Dime);
  value_in_cents(Coin::Quarter(UsState::Alabama));
}

fn value_in_cents(coin: Coin) -> u32 {
  // Cuando match se ejecuta, compara el valor resultante contra el patrón de cada brazo, en orden. Si un
  // patrón coincide con el valor, se ejecuta el código asociado a ese patrón. Si ese patrón no coincide
  // con el valor, la ejecución continúa hasta el siguiente brazo
  // Las llaves no se usan si el código del brazo es corto. Si es necesario ejecutar múltiples líneas de
  // código en un brazo se pueden usar llaves.
  match coin {
    Coin::Penny => {
      println!("Penny");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    // match es que pueden enlazar con las partes de los valores
    // que coinciden con el patrón. Así es como podemos extraer valores de las variantes enum.
    Coin::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    }
  }
}