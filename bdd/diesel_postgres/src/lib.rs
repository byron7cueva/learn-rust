#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schemas;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL se debe configurar");

    PgConnection::establish(&database_url)
    .expect(&format!("Error conectando a {}", database_url))
}