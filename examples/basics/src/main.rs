mod fibonacci;
mod temperatura;
mod adivinanza;
mod ownership;

use crate::temperatura::{celsius_to_fahrenheit, fahrenheit_to_celsius};
use crate::fibonacci::calcular_fibonnaci;
// use crate::adivinanza::adivinar;
use adivinanza::adivinar;
use crate::ownership::{test_stack, test_heap};

fn main() {
    let celsius: f64 = 37.;
    let result_fahrenheit = celsius_to_fahrenheit(&celsius);
    println!("La temperatura {}°C es {}°F", celsius, result_fahrenheit);
    let resul_celsius = fahrenheit_to_celsius(&result_fahrenheit);
    println!("La temperatura en {}°F es {}°C", result_fahrenheit, resul_celsius);

    let n = 4;
    let result_fibonnacci = calcular_fibonnaci(n);
    println!("El fibonacci es {}", result_fibonnacci);

    adivinar();

    //Ownership
    //test_stack();
    // test_heap();
}
