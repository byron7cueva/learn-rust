const SCALE: f64 = 32.;
const INCRE_SCALE: f64 = 9. / 5.;
const DECRE_SCALE: f64 = 5. / 9.;

pub fn celsius_to_fahrenheit(celsius: &f64) -> f64 {
    (celsius * INCRE_SCALE) + SCALE
}

pub fn fahrenheit_to_celsius(fahrenheit: &f64) -> f64 {
    (fahrenheit - SCALE) * DECRE_SCALE
}
