# Rust

Rust es un lenguaje compilado

Tiene tres canales: Release, Beta y Nigthly. www.whatrustisit.com, para ver las versiones disponibles en cada canal.
*Creat* Se les llama a las dependencias

*rustfmt:* Es el formateador de rust.
*Clippy:* Es el linter de Rust.

## Cargo

Es el gestor de paquetes y sistema de compilacion de Rust.
Permite crear nuevos proyectos. Se debe nombrar a un proyecto con minúsculas y separado con raya baja.
Actualizar las dependencias
Correr pruebas unitarias y de integracion
Publicar el paquete en creats
Documentacion: doc.rust-lang.org/cargo/commands/index.html

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

#### Test

Correr un test

```bash
cargo test
```

#### Documentacion

Generar documentacion

```bash
cargo doc
```

#### Publicar

Publicar crate

```bash
cargo publ
```

#### Nuevo Proyecto

Creando un proyecto bin

```bash
cargo new nombre_proyecto
```

Creando un proyecto lib

```bash
cargo new nombre_lib_project --lib
```


## Rust Playground

Es una aplicacion web donde permite probar codigo de Rus, tambipen compartir código.
Tambien nos permite acceder a los 100 creats mas descargados.

*Miri* Para buscar problemas de undefined behavior.
*Expand macros:* Permiten ver las macros y ver como se comportan.

## Rustup

Administra la version de rust que se esta utilizando (Toolchain)

### Instalar una version

```bash
rustup install *.*.*
```

Instalar una version beta

```bash
rustup install beta
```

Instalar una version nightly

```bash
rustup install nightly
```

Instalar una version estable

```bash
rustup install stable
```

### Cambiar version por defecto

Cambiar la version que se esta utilizando por defecto

```bash
rustup default *.*.*
```

### Desinstalar

Desinstalara una version

```bash
rustup uninstall *.*.*
```