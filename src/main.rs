mod cors;
mod db;
mod endpoint_models;
mod entities;
mod json_date;
mod models;
mod rocket;
mod schema;
mod inserteable_models;

fn main() {
    rocket::main()
}
