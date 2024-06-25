use std::borrow::Borrow;

use rocket::serde::json::Json;
use rocket::*;

use crate::{
    db::establish_connection,
    endpoint_models,
    entities::{dish, menucard, setting, tag},
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

#[get("/dish/<id>?<query..>")]
fn get_dish(id: i32, query: QueryParams) -> Json<endpoint_models::Dish> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let dish = dish::get_dish(&mut establish_connection(), id, &expands, None).unwrap();
    Json::from(dish)
}

#[get("/dish?<query..>")]
fn get_all_dishes(query: QueryParams) -> Json<Vec<endpoint_models::Dish>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let dishes = dish::get_all_dishes(&mut establish_connection(), &expands).unwrap();
    Json::from(dishes)
}

#[get("/tag/<id>?<query..>")]
fn get_tag(id: i32, query: QueryParams) -> Json<endpoint_models::Tag> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let tag = tag::get_tag(&mut establish_connection(), id, &expands).unwrap();
    Json::from(tag)
}

#[get("/tag?<query..>")]
fn get_all_tags(query: QueryParams) -> Json<Vec<endpoint_models::Tag>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let tags = tag::get_all_tags(&mut establish_connection(), &expands).unwrap();
    Json::from(tags)
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
            get_all_dishes,
            get_tag,
            get_all_tags,
            get_table,
            get_reservation,
            get_person,
        ],
    )
}
