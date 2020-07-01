// Contantes
// Rust tiene dos tipos diferentes de constantes que se pueden declarar en cualquier ámbito, incluido el
// global. Ambos requieren una anotación explícita

// const
// Un valor inmutable. Esta se la utiliza mas comumente
const THRESHOLD: i32 = 10;

// static
// Una variable posiblemente mutable con una vida útil estática. La vida útil estática se
// infiere y no es necesario especificarla. Acceder o modificar una variable estática mutable no
// es seguro
static LANGUAGE: &str = "Rust";

fn main () {
    let n = 6;
    println!("Este lenguage es {}", LANGUAGE);
    println!("El límite es {}", THRESHOLD);
    println!("{} es {}", n, if is_big(n) { "mayor" } else { "menor" });
}

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}