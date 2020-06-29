extern crate diesel_postgres;
extern crate diesel;

use self::diesel_postgres::*;
use diesel::prelude::*;

fn main () {
  let connection = establish_connection();
}