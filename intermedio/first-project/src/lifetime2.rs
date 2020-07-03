// Lifetimes

// Cada referencia en Rust tiene una vida útil, que es el alcance para el cual esa referencia es válida
// La mayoría de las veces, los tiempos de vida son implícitos e inferidos, al igual que la mayoría de las
// veces, los tipos son inferidos

// Debemos anotar los tipos cuando hay varios tipos posibles. De manera similar, debemos anotar los
// tiempos de vida cuando los tiempos de vida de las referencias podrían estar relacionados de
// diferentes maneras.

// Rust requiere que anotemos las relaciones usando parámetros genéricos de vida para asegurar que
// las referencias reales utilizadas en tiempo de ejecución sean definitivamente válidas

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest2(string1.as_str(), string2);
    println!("La cadena mas larga es: {}", result);

    // crea una instancia de la struct ImportantExcerpt que contiene una referencia a la
    // primera frase de la cadena propiedad de la variable novel. Los datos en novel existen antes de que
    // se cree la instancia de ImportantExcerpt. Además, novel no sale del ámbito de aplicación antes de
    // ImportantExcerpt, por lo que la referencia en la instancia de ImportantExcerpt es válida
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };

    // Lifetime static
    // Una vida especial es 'static que denota la duración completa del programa.
    // El texto de esta cadena se almacena directamente en el binario de la aplicación, que siempre está
    // disponible. Por lo tanto, la vida útil de todas las cadenas literales es ‘static
    // Antes de especificar 'static como la vida útil de una referencia, piensa si la referencia vive toda la
    // vida del programa o no. La mayoría de las veces, el problema es el resultado de intentar crear una
    // referencia colgante o un desajuste de la vida útil disponible. En tales casos, la solución es arreglar
    // esos problemas no especificar ‘static sin más
    let s: &'static str = "I have a static lifetime.";
}

// La funcion genera un error de tipo lifetime
// El tipo de retorno necesita un parámetro genérico de vida porque Rust
// no puede decir si la referencia que se devuelve se refiere a x o y
// Cuando definimos la función no sabemos la duración
// concreta de las referencias que se pasarán,

// El verificador de préstamos tampoco puede determinar esto, porque no sabe cómo se relacionan los
// tiempos de vida de x e y con el tiempo de vida del valor de retorno.
/* fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} */

// Para corregir este error, añadiremos parámetros genéricos de vida que definen la relación entre
// las referencias para que el verificador de préstamos pueda realizar su análisis

// Las lifetimes no cambian el tiempo de vida de ninguna de las referencias
// Las funciones pueden aceptar referencias con cualquier vida especificando un parámetro de vida
// genérico. Las lifetimes describen las relaciones de las vidas de múltiples referencias entre sí sin
// afectar las vidas

// Los nombres de los parámetros de lifetimes deben comenzar con un apóstrofe (') y suelen ser
// todos en minúsculas y muy cortos
// Colocamos anotaciones de parámetros de vida después de la & de una referencia, usando un espacio
// para separar la anotación del tipo de referencia
// &i32 // referencia a un i32 sin un parámetro de vida
// &'a i32 // una referencia a un i32 que tiene un parámetro de vida llamado 'a
// &'a mut i32 una referencia mutable a un i32 que también tiene el parámetro de vida 'a

// Una anotación de vida por sí sola no tiene mucho significado, porque las anotaciones están
// destinadas a indicar a Rust cómo se relacionan entre sí los parámetros genéricos de vida de
// múltiples referencias

// Lifetimes en las definiciones de función
// La restricción que queremos expresar en esta firma es que todas las referencias en los
// parámetros y el valor de retorno deben tener la misma duración
// La vida útil genérica 'a obtendrá la vida útil concreta que es igual a la menor de las vidas
// útiles de x e y. Debido a que hemos anotado la referencia devuelta con el mismo parámetro de
// vida útil 'a, la referencia devuelta también será válida para la duración de la menor de las
// vidas útil de x e y
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Cuando se devuelve una referencia desde una función, el parámetro de duración del tipo de retorno
// debe coincidir con el parámetro de duración de uno de los parámetros
fn longest3<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Aquí aunque hemos especificado un parámetro de vida 'a para el tipo de retorno, esta
// implementación fallará en la compilación porque el valor de retorno de la vida útil no está
// relacionado con la vida útil de los parámetros en absoluto
// El problema es que el resultado se sale del alcance y se limpia al final de la función longest
// En este caso, la mejor solución sería devolver un tipo de datos propio en lugar de una
// referencia, por lo que la función de llamada es responsable de limpiar el valor
/* fn longest4<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
} */

// La sintaxis de vida útil se refiere a la conexión de la vida útil de varios
// parámetros y los valores de retorno de las funciones

// Lifetimes en definición de structs
// Esta estructura tiene un campo, part, que contiene una slice de cadena, que es una referencia
// Esta anotación significa que una instancia de ImportantExcerpt no puede sobrevivir a
// la referencia que tiene en su campo part
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetimes en definición de métodos
// En las firmas de métodos dentro del bloque impl, las referencias pueden estar vinculadas a la vida
// útil de las referencias en los campos de la estructura, o pueden ser independientes
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Aqui se aplica la tercera regla de elison
    // Entonces, debido a que uno de los parámetros es &self, el tipo de retorno obtiene la vida
    // de &self, y todas las vidas han sido calculadas.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}