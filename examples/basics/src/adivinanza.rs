// Permite a Rust saber que usaremos la crate rand como una dependencia externa.
// Esto también equivale a llamar a use rand, así que ahora podemos llamar a
// cualquier cosa en la crate rand colocando rand::: delante de ella.
// extern crate rand; Ya no se necesita utilizar extern crate desde la version 2018
// Incluyendo la libreria io en el ambito de la aplicacion
// La biblioteca io proviene de la bibliotecs
// use sirve para incluir en el ámbito de aplicación explícitamente un tipo que se desee utilizar
use std::io::stdin;

//El trait Rng define los métodos que los
// generadores de números aleatorios implementan, y este trait debe estar en el ámbito de aplicación
// para que podamos usar esos métodos
use rand::Rng;

// Es un enumerador que tiene como variantes Less, Greater y Equal
use std::cmp::Ordering;

pub fn adivinar() {
    println!("¡Adivina un número!");

    // Genrando número aleatorio entre 1 a 100
    let numero_secreto = rand::thread_rng().gen_range(1, 101);
    // println!("El numero es: {}", numero_secreto);
    loop {
        println!("Ingresa un número:");
        // String es un tipo de cadena proporcionada por la biblioteca estándar que es texto ampliable y codificado en UTF-8.
        // La sintaxis :: indica que new es una función asociada del tipo String. Una función
        // asociada se implementa en un tipo, en lugar de en una instancia particular de un
        // String. Algunos lenguajes lo llaman un método estático.
        let mut input = String::new();
        // Manejador de entrada estándar, para tener información del usuario
        // Retorna un io::Result,
        // Rust tiene una serie
        // de tipos denominados Result en su biblioteca estándar: un Result genérico así como versiones
        // específicas para submódulos, como io::Result.
        // Los tipos Result son enumeraciones, es un tipo que puede tener un conjunto fijo de valores,
        // y esos valores se denominan variantes de la enumeración
        // Para Result las variantes son Ok o Err. La variante Ok indica que la operación fue exitosa, y dentro
        // de Ok está el valor generado con éxito. La variante Err significa que la operación falló, y Err
        // contiene información sobre cómo o por qué falló la operación.
        // El propósito de estos tipos de resultados es codificar la información sobre el manejo de errores. Los
        // Result, tienen métodos definidos en ellos. Una instancia
        // de io::Result tiene un método expect al que puede llamar
        stdin().read_line(&mut input)
            .expect("Fallo al leer la linea"); // Si da error se mostrar este mensaje y se bloqueara el programa

        // Shadowing
        // Sombreando el valor anterior de input con un nuevo
        // Esta característica se utiliza a menudo en situaciones en las que se desea convertir un valor de un tipo a
        // otro y mantener el mismo nombre de la variable
        // trim elimina los espacios en blanco y el \n resultado de dar enter
        // parse() convierte a número, el tipo se indica con la notación del tipo. Este devuelve un Result
        // Estrellandose al error haciendo que pare
        /* let input: u32 = input.trim().parse()
            .expect("Por favor ingresa un número");*/

        // Manejando el error
        // parse() devuelve un enumerado Result que tiene como variante Ok y Err
        let input:u32 = match input.trim().parse() {
            // Ok en el caso que se ingreso un numero
            Ok(num) => num,
            // Err cuando no se genero un error
            // _ es un valor comodín; estamos diciendo que queremos igualar todos los valores Err, sin importar la
            // información que tengan dentro.
            Err(_) => continue // Ignorando el error y continuando la siguiente iteración
        };

        println!("Tu ingresaste {}", input);

        // El metodo cmp compara dos valores y puede ser utilizado en cualquier cosa que puede ser comparada
        // La expresión match consta de ramas
        // Una rama consiste en un código que debe ejecutarce si un valor dado al inicio de la expreción coincide
        // con el patrón de esa rama y la expresión match termina
        match input.cmp(&numero_secreto) {
            Ordering::Less => println!("Es menor"),
            Ordering::Greater => println!("Es mayor"),
            Ordering::Equal => {
                println!("Es igual ganaste");
                break;
            }
        }
    }
}