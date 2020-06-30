// Struct
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool
}

fn main () {
  let user1 = User {
    email: String::from("user1@mail.com"),
    username: String::from("user1"),
    sign_in_count: 1,
    active: true
  };

  // La sintaxis .. especifica que los campos restantes que no se han fijado explícitamente deben
  // tener el mismo valor que los campos de la instancia indicada.
  let user2 = User {
    email: String::from("user2@mail.com"),
    username: String::from("username"),
    ..user1
  };
}

// Construye un Usuario
fn build_user(email: String, username: String) -> User {
  // Debido a que el campo email y el parámetro email tienen el mismo nombre, sólo
  // necesitamos escribir email en lugar de email: email
  User {
    email,
    username,
    active: true,
    sign_in_count: 1
  }
}