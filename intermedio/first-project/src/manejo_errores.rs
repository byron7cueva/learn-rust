// Manejo de errores
// Rust no tiene excepciones. En su lugar, tiene el tipo Result<T, E> para errores recuperables y
// la macro panic! que detiene la ejecución cuando el programa encuentra un error irrecuperable

use std::{io, io::ErrorKind, fs::File};
use std::io::Read;

fn main() {
    // ERRORES NO RECUPERABLES
    //Cuando se ejecute la macro panic!, el programa imprimirá un
    // mensaje de error, desenrrollará y limpiará la pila para terminar. Esto ocurre cuando se ha detectado
    // un error de algún tipo y no está claro para el programador cómo manejar el error

    // panic!("Ocurrio un error");

    let v = [1, 2, 3];
    println!("Introduce el indice:");
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    match input.trim().parse::<usize>() {
        Ok(n) => println!("{}", v[n]),
        Err(e) => println!("Error")
    }

    // Establecer la variable de entorno RUST_BACKTRACE
    // para obtener un rastreo exacto de lo que causó el error
    // RUST_BACKTRACE=1 cargo run

    // ERRORES RECUPERABLES
    // Manejo de fallos potenciales con el tipo Result, el enum Result
    // se define como dos variantes, Ok y Err
    let f = File::open("file.txt");

    // Deteniendo el programa en el caso de existir un error
    /* let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Ocurrio un problema al abrir el archivo: {:?}", error),
    }; */

    // Decidiendo que hacer dependiendo del tipo de error
    let f = match f {
        Ok(file) => file,
        // Validando el tipo de error
        Err(error) => match error.kind() {
            // Si no existe se lo crea
            // File::create también
            // podría fallar, necesitamos añadir otra expresión match
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Ocurrio un error al crear el archivo: {:?}", e)
            },
            // Si hay otro tipo de error se detiene
            other_error => panic!("Ocurrio un problema abriendo el archivo: {:?}", other_error)
        }
    };

    // Match es muy poderoso, pero también muy primitivo
    // Atajos: unwrap y expect
    // Si el valor de Result es Ok, unwrap devolverá el valor dentro de Ok. Si el
    // resultado es Err, unwrap llamará a la macro panic!
    // let f2 = File::open("file2.txt").unwrap();

    // expect. Es similar a similar a unwrap, pero este nos permite elegir el mensaje de panic!
    // let f2 = File::open("file2.txt").expect("Error al abrir el archivo");

    read_user_name_from_file();
    read_user_name_from_file2();
    read_user_name_from_file3();

    // El operador ? sólo se puede utilizar en funciones que devuelven Result
    // let f = File::open("file4.txt")?; // Esto genera un error ya que la funcion main retorna ()
}

// Propagación de errores de manera dificil
fn read_user_name_from_file() -> Result<String, io::Error> {
    let f = File::open("file3.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// Un atajo para propagar errores: el operador ?. Manera facil
// El ? después de un valor Result funciona casi de la misma manera que las expresiones match del
// ejemplo anterior. Si se obtiene un Err será devuelto como si utilizaramos un return. Colocamos el
// Ok al final del cuerpo de la función para que sea devuelto caso de producirse todo sin problemas
fn read_user_name_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("file3.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Se puede acortar aún más este código encadenando las llamadas del método inmediatamente
// después de ?
fn read_user_name_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("file3.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Sin embargo, la función principal puede devolver un Resultado<T, E>
// Box<dyn Error> se llama "trait objet"
// Box<dyn Error> indica "cualquier tipo de error"
/* fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
} */