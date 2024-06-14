use db::{establish_connection, get_settings};
use diesel::PgConnection;
use models::*;

mod db;
mod endpoint_models;
mod json_date;
mod models;
mod rocket;
mod schema;

fn main() {
    let db_connection = establish_connection();
    get_settings();
}
