// if let
// Permite manejar valores que coinciden con un patrón mientras ignoras el resto.
// La sintaxis if let toma un patrón y una expresión separados por un signo igual. Funciona de la
// misma manera que un match donde la expresión es el match y el patrón es su primer brazo.
// Sin embargo, se pierde el control exhaustivo que hace cumplir la coincidencia, que tiene match
// Se puede pensar en if let si solo hay una coincidencia que ejecuta código cuando el
// valor coincide con un patrón y luego ignora todos los demás valores.

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
    // Queremos hacer algo con la coincidencia Some(3) pero no hacer nada con ningún otro valor
    // Some<u8> o el valor None. Para satisfacer la expresión match tenemos que añadir _ => () después
    // de procesar sólo una variante
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("Three"),
        _ => ()
    }

    // Lo anterior se puede escribir esto de una manera más corta usando if let
    if let Some(3) = some_u8_value {
        println!("Three")
    }

    //
    let coin = Coin::Penny;
    let mut count = 0;

    match coin {
        Coin::Quarter(state) => println!("El quarter de esta {:?}", state),
        _ => count += 1
    }

    // Podemos incluir una else con un if let. El bloque de código que va con el resto es el mismo que el
    // bloque de código que iría con el caso _ en la expresión match que es equivalente a if let y else
    let coin2 = Coin::Penny;
    if let Coin::Quarter(state) = coin2 {
        println!("El quarter de esta {:?}", state);
    } else {
        count += 1;
    }
}