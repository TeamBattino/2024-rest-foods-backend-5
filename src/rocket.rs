use std::borrow::Borrow;

use rocket::serde::json::Json;
use rocket::*;

use crate::{
    db::establish_connection, endpoint_models,
    entities::menucard::get_menucard as get_menucard_by_id,
};

#[get("/setting?<name>")]
fn get_setting(name: Option<Vec<String>>) -> String {
    let name = name.unwrap_or_default();
    let name = name.join(", ");
    format!("Names: {}", name)
}
/* #[get("/menucard/<id>?<expands>")]
fn get_menucard(id: i32, expands: Option<Vec<&str>>) -> Json<endpoint_models::Menucard> {
    println!(
        "Expansions: {}",
        expands.clone().unwrap_or_default().join(", ")
    );
    let menucard =
        get_menucard_by_id(&mut establish_connection(), id, &expands.unwrap_or(vec![])).unwrap();
    Json::from(menucard)
} */

#[derive(FromForm)]
struct QueryParams {
    expands: Option<Vec<String>>,
}

#[get("/menucard/<id>?<query..>")]
fn get_menucard(id: i32, query: QueryParams) -> Json<endpoint_models::Menucard> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let menucard = get_menucard_by_id(&mut establish_connection(), id, &expands).unwrap();
    Json::from(menucard)
}

#[get("/dish")]
fn get_dish() -> &'static str {
    "Dish"
}
#[get("/tag")]
fn get_tag() -> &'static str {
    "Tag"
}

#[get("/table")]
fn get_table() -> &'static str {
    "Table"
}

#[get("/reservation")]
fn get_reservation() -> &'static str {
    "Reservation"
}

#[get("/person")]
fn get_person() -> &'static str {
    "Person"
}

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_setting,
            get_menucard,
            get_dish,
            get_tag,
            get_table,
            get_reservation,
            get_person,
        ],
    )
}
