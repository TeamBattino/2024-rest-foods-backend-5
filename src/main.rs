use std::collections::HashSet;

use db::{establish_connection, get_db_settings};
use diesel::PgConnection;

use models::*;
mod db;
mod endpoint_models;
mod entities;
mod json_date;
mod models;
mod rocket;
mod schema;

fn main() {
    let mut db_connection = establish_connection();
    rocket::main()
}
