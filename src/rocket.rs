//! # API Documentation
//!
//! This module contains the implementation of several REST API endpoints using the Rocket framework and OpenAPI integration through `rocket_okapi`.
//!
//! ## Endpoints
//!
//! The following endpoints are defined:
//!
//! - `GET /setting`
//! - `GET /menucard/<id>`
//! - `GET /menucard`
//! - `GET /dish/<id>`
//! - `GET /dish`
//! - `GET /tag/<id>`
//! - `GET /tag`
//! - `GET /table`
//! - `GET /reservation`
//! - `GET /person`
//! - `POST /person`
//! - `POST /reservation`
//!
//! ## Query Parameters
//!
//! All endpoints support the `expands` query parameter, which can be used to specify related entities to expand in the response.

use std::borrow::Borrow;

use rocket::serde::json::Json;
use rocket::*;
use rocket_okapi::{
    openapi, openapi_get_routes,
    rapidoc::{make_rapidoc, GeneralConfig, HideShowConfig, RapiDocConfig},
    settings::UrlObject,
    swagger_ui::{make_swagger_ui, SwaggerUIConfig},
};
use schemars::JsonSchema;

use crate::{
    cors::Cors,
    db::establish_connection,
    endpoint_models,
    entities::{
        dish, menucard, person::insert_person, reservation::insert_reservation, setting, tag,
    },
    inserteable_models, models,
};

/// Query parameters that can be used in the API requests.
///
/// The `expands` parameter is used to specify related entities to expand in the response.
#[derive(FromForm, JsonSchema)]
struct QueryParams {
    /// List of entities to expand.
    expands: Option<Vec<String>>,
}

/// Get the setting.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the setting.
///
/// # Example
///
/// ```
/// GET /setting?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get a menucard by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the menucard.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the menucard.
///
/// # Example
///
/// ```
/// GET /menucard/1?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get all menucards.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of menucards.
///
/// # Example
///
/// ```
/// GET /menucard?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get a dish by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the dish.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the dish.
///
/// # Example
///
/// ```
/// GET /dish/1?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get all dishes.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of dishes.
///
/// # Example
///
/// ```
/// GET /dish?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get a tag by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the tag.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the tag.
///
/// # Example
///
/// ```
/// GET /tag/1?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get all tags.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of tags.
///
/// # Example
///
/// ```
/// GET /tag?expands=entity1,entity2
/// ```
#[openapi]
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

/// Get the table information.
///
/// # Returns
///
/// - `200 OK`: Returns the table information.
///
/// # Example
///
/// ```
/// GET /table
/// ```
#[openapi]
#[get("/table")]
fn get_table() -> &'static str {
    "Table"
}

/// Get the reservation information.
///
/// # Returns
///
/// - `200 OK`: Returns the reservation information.
///
/// # Example
///
/// ```
/// GET /reservation
/// ```
#[openapi]
#[get("/reservation")]
fn get_reservation() -> &'static str {
    "Reservation"
}

/// Get the person information.
///
/// # Returns
///
/// - `200 OK`: Returns the person information.
///
/// # Example
///
/// ```
/// GET /person
/// ```
#[openapi]
#[get("/person")]
fn get_person() -> &'static str {
    "Person"
}

/// Create a new person.
///
/// # Body Parameters
///
/// - `person`: JSON object containing the person details.
///
/// # Returns
///
/// - `201 Created`: Returns the created person.
///
/// # Example
///
/// ```
/// POST /person
/// Content-Type: application/json
///
/// {
///     "name": "John Doe",
///     "email": "john.doe@example.com"
/// }
/// ```
#[openapi]
#[post("/person", format = "json", data = "<person>")]
fn post_person(person: Json<inserteable_models::Person>) -> Json<models::Person> {
    let response = insert_person(person);
    Json(response)
}

/// Create a new reservation.
///
/// # Body Parameters
///
/// - `reservation`: JSON object containing the reservation details.
///
/// # Returns
///
/// - `201 Created`: Returns the created reservation.
///
/// # Example
///
/// ```
/// POST /reservation
/// Content-Type: application/json
///
/// {
///     "person_id": 1,
///     "table_id": 2,
///     "time": "2024-06-25T19:00:00Z"
/// }
/// ```
#[openapi]
#[post("/reservation", format = "json", data = "<reservation>")]
fn post_reservation(
    reservation: Json<inserteable_models::Reservation>,
) -> Json<models::Reservation> {
    let response = insert_reservation(reservation);
    Json(response)
}

/// Launch the Rocket application with the defined routes and configurations.
///
/// # Returns
///
/// A configured Rocket instance.
#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount(
            "/",
            openapi_get_routes![
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
                post_person,
                post_reservation,
            ],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/rapidoc/",
            make_rapidoc(&RapiDocConfig {
                general: GeneralConfig {
                    spec_urls: vec![UrlObject::new("General", "../openapi.json")],
                    ..Default::default()
                },
                hide_show: HideShowConfig {
                    allow_spec_url_load: false,
                    allow_spec_file_load: false,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )
}
