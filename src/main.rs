use db::get_settings;
use models::*;

mod db;
mod models;
mod schema;
mod rocket;

fn main() {
    get_settings();
    }