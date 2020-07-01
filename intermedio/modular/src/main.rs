// Usando un punto y coma después del sonido mod en lugar de un bloque le dice a Rust que cargue el
// contenido del módulo desde otro archivo con el mismo nombre que el módulo
mod sound;
fn main() {
    // Absolute path
    crate::sound::instrument::clarinet();

    // Relative path
    sound::instrument::clarinet();
}