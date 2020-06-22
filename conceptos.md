# Rust

Rust es un lenguaje compilado

Tiene tres canales: Release, Beta y Nigthly. www.whatrustisit.com, para ver las versiones disponibles en cada canal.
*Creat* Se les llama a las dependencias

*rustfmt:* Es el formateador de rust.
*Clippy:* Es el linter de Rust.

## Cargo

Es el gestor de paquetes y sistema de compilacion de Rust.

### Estructura

*src:* Código fuente de la aplicacion.
*src/main.rs* El punto de entrada de la aplicacion.
*Cargo.toml* Es un fichero TOML de configuración del proyecto. Contiene la description del proyecto y la configuracion de las dependencias.
*Cargo.lock* Es un fichero donde se va dejar declarado que versiones especificas de las dependencias hemos utilizado a la hora de compilar y se van quedar conjeladas.

### Comandos

#### Compilar

```bash
cargo build
```

Compila y genera una carpeta target y un ejecutable.

#### Run

```bash
cargo run
```

Compila y corre el programa.


#### Check


```bash
cargo check
```

Es similar a build con la diferencia que no genera un ejecutable. Esto pude ser util para comprobar que nuestro código esta bien.

#### Release

```bash
cargo build --release
```

Generar con optimizaciones para ser desplegado. Genera otra carpeta release

```bash
cargo run --release
```

Correr en modo release


## Rust Playground

Es una aplicacion web donde permite probar codigo de Rus, tambipen compartir código.
Tambien nos permite acceder a los 100 creats mas descargados.

*Miri* Para buscar problemas de undefined behavior.
*Expand macros:* Permiten ver las macros y ver como se comportan.