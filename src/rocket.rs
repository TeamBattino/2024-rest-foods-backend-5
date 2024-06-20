use std::borrow::Borrow;

use rocket::serde::json::Json;
use rocket::*;

use crate::{
    db::establish_connection,
    endpoint_models,
    entities::{menucard, setting},
};

#[get("/setting?<query..>")]
fn get_setting(query: QueryParams) -> Json<endpoint_models::Setting> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let setting = setting::get_setting(&mut establish_connection(), &expands).unwrap();
    Json::from(setting)
}

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

    let menucard = menucard::get_menucard(&mut establish_connection(), id, &expands).unwrap();
    Json::from(menucard)
}

#[get("/menucard?<query..>")]
fn get_all_menucards(query: QueryParams) -> Json<Vec<endpoint_models::Menucard>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let menucard = menucard::get_all_menucards(&mut establish_connection(), &expands).unwrap();
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
            get_all_menucards,
            get_dish,
            get_tag,
            get_table,
            get_reservation,
            get_person,
        ],
    )
}
