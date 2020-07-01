// Enum Option
// Permite codificar el escenario muy común en el que un valor podría ser algo o podría ser nada
// Rust no tiene la
// propiedad Null. Null es un valor que significa que no hay valor. En lenguajes con nulo, las variables
// siempre pueden estar en uno de dos estados: nulo o no nulo.

// Rust no tiene nulos, pero sí tiene una enumeración que puede codificar el concepto de que un valor
// está presente o ausente.
// Esta enumeración es Option<T>, y está definida por la librería estándar

// Sus variantes: puedes usar Some y None directamente sin el prefijo Option::
// la variante Some del enum Option puede contener una pieza de datos de cualquier tipo
// al utilizar genéricos
fn main () {
  let some_number = Some(5);
  let some_string = Some("a string");
  // Si usamos None en vez de Some, necesitamos decirle a Rust qué tipo de Opción<T> tenemos,
  // porque el compilador no puede inferir el tipo que tendrá la variante Some mirando sólo a un valor
  // None.
  // Cuando tenemos un valor None, en cierto sentido, significa lo mismo que nulo: no tenemos un
  // valor válido
  let absent_number: Option<i32> = None;

  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  // No se puede sumar un i8 y un Option<i8>, porque son tipos diferentes
  // let sum = x + y; // ste código no se compila porque está intentando añadir un i8 a un Option<i8>
  // Se debe convertir una Option<T> en una T antes de poder realizar operaciones T
  // Entonces, cuando se usa ese valor, se requiere que se maneje explícitamente el caso cuando el valor es nulo
  // Dondequiera que un valor tenga un tipo que no sea una Option<T>, puedes asumir con seguridad que el valor no es nulo.

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);
}

// Option con match
fn plus_one(x: Option<i32>) -> Option<i32> {
  // Los match en Rust son exhaustivos: debemos agotar todas las posibilidades para que el código sea válido.
  // Especialmente en el caso de Option<T>, cuando Rust nos impide olvidarnos de manejar
  // explícitamente el caso de None, nos protege de asumir que tenemos un valor cuando podríamos
  // tener nulo
  match x {
    None => None,
    // Obteniendo el valor de T de Option<T>
    Some(i) => Some(i + 1)
  }
}