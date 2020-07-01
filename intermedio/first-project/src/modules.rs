// Modules
// Los módulos nos permiten organizar el código en grupos.
// Los módulos son el límite de privacidad en Rust
// En la sección "Paquetes y Crates para hacer librerías y ejecutables" mencionamos que src/main.rs y
// src/lib.rs se llaman raíces de cajas. Se llaman raíces de caja porque el contenido de cualquiera de
// estos dos archivos forma un módulo llamado crate en la raíz del árbol de módulos de la caja.
// Todo el árbol de módulos está ubicado bajo el módulo implícito llamado crate

mod sound {
    fn guitar() {}
}

// Para organizar el código en una jerarquía de módulos, puedes anidar módulos dentro de otros
// módulos
mod sound2 {
    pub mod instrument {
        pub mod woodwind {
            pub fn clarinet() {
                println!("call to clarinet");
                // Podemos usar super para ir al módulo padre
                super::breathe_in();
            }
        }

        fn breathe_in() {
            println!("call breathe_in")
        }
    }
    mod voice {

    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: u32
    }

    impl Vegetable {
        // Debido a que plant::Vegetable tiene un campo privado, la estructura necesita proporcionar una
        // función pública asociada que construya una instancia de Vegetable Si Vegetable no tuviera tal
        // función, no podríamos crear una instancia de Vegetable en main porque no se nos permite establecer
        // el valor del campo id
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1
            }
        }
    }
}

mod menu {
    // Si se hace una enum pública, todas sus variantes son públicas. Sólo necesita el pub
    // antes de la palabra clave enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// llevar un path a un ámbito una vez y luego llamar a los elementos de ese path como si fueran elementos
// locales: con la palabra clave use
// Añadir use y una ruta en un ámbito es similar a crear un enlace simbólico en el sistema de ficheros
// La mayoría de programadores tiende a especificar rutas absolutas comenzando con crate
// porque es más probable que los elementos de definición y llamada de código se muevan alrededor
// del árbol de módulos de forma independiente entre sí
// use crate::sound2::instrument::woodwind;
// Puede que en el futuro no sea necesario iniciar rutas relativas con self; es una inconsistencia en el
// lenguaje que se está tratando de eliminar.
// use self::sound2::instrument::woodwind;
use  sound2::instrument::woodwind;

// Para las funciones, se considera más correcto especificar el módulo
// padre de la función con use, y luego especificar el módulo padre cuando se llama la función. Deja
// claro que la función no está definida en main.
// use crate::sound2::instrument::woodwind; // correcto
// use crate::sound::instrument::woodwind::clarinet; // No esta correcto
// Para estructuras, enums y otros elementos se especifica la ruta completa con el elemento:
// use std::collections::HashMap;

// Podemos especificar un nuevo nombre local para el tipo añadiendo as y un nuevo nombre después de use
use std::fmt::Result;
use std::io::Result as IoResult;

fn main () {
    // Ruta absoluta
    crate::sound2::instrument::woodwind::clarinet();

    // Ruta relativa
    sound2::instrument::woodwind::clarinet();

    let mut veg = plant::Vegetable::new("tomate");
    veg.name = String::from("cebolla");
    println!("El nombre del vegetal es {}", veg.name);
    // println!("El id del vegetal es {}", veg.id); No se puede acceder a la propiedad id ya que es privada

    let order1 = menu::Appetizer::Salad;
    let order2 = menu::Appetizer::Soup;

    woodwind::clarinet();
}