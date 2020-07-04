#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv;
use std::env;

mod schemas;
mod models;
use self::schemas::posts::{self as post_schema, dsl};
use self::models::{Post, NewPost};

fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL se debe configurar");

    println!("La cadena de conexion es {}", database_url);
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error conectandose a la base de datos: {}", database_url));

    println!("Se paso de la conexion");

    /* let other_title = "Segundo post";
    let other_body = "Segundo contenido";
    let post = create_post(&connection, other_title, other_body); */

    // update_post(&connection, 3);
    delete_post(&connection, 2);

    let results = dsl::posts.filter(dsl::published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading post");

    println!("El total de post son: {}", results.len());

    for post in results {
        println!("{:?}", post);
    }
}

fn create_post<'a>(connec: &PgConnection, title: &'a str, body: &'a str) -> Post {
    let new_post = NewPost { title, body };
    diesel::insert_into(post_schema::table)
        .values(&new_post)
        .get_result(connec)
        .expect("Error a guardar un post")
}

fn update_post(connec: &PgConnection, id: i32) {
    diesel::update(dsl::posts.find(id))
        .set(dsl::published.eq(true))
        .get_result::<Post>(connec)
        .expect(&format!("No se encontro el post con el id: {}", id));
}

fn delete_post(connec: &PgConnection, id: i32) {
    diesel::delete(dsl::posts.find(id))
        .execute(connec)
        .expect("Error al eliminar el post");
}