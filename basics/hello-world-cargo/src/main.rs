fn main () {
  let number = 2;

  // Funciona similar al switch
  match number {
    // Si number es igual a 1
    1 => println!("It is one"),
    2 => println!("It is two"),
    // Si number es igual a 10 o 11
    10 | 11 => println!("It is ten or eleven"),
    // Cuando no cumple algun caso
    20...30 => println!("It is between 20 to 30"), // Esto ya esta deprecado
    _ => println!("It doesn't match")
  }

  let name = "Byron";

  match name {
    "Luis" => println!("Your name is Luis"),
    "Carlos" => println!("Your name is Carlos"),
    _ => println!("Don't know your name")
  }
}