//! # API Documentation
//!
//! This module contains the implementation of several REST API endpoints using Rocket framework and OpenAPI integration through `rocket_okapi`.
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
//! - `GET /table/<id>`
//! - `GET /table`
//! - `GET /reservation/<id>`
//! - `GET /reservation`
//! - `GET /person/<id>`
//! - `GET /person`
//! - `POST /person`
//! - `POST /reservation`
//!
//! ## Query Parameters
//!
//! All endpoints support the `expands` query parameter, which can be used to specify related entities to expand in the response.

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
    entities::{dish, menucard, person, reservation, setting, table, tag}, models,
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

/// Get a table by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the table.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the table.
///
/// # Example
///
/// ```
/// GET /table/1?expands=entity1,entity2
/// ```
#[openapi]
#[get("/table/<id>?<query..>")]
fn get_table(id: i32, query: QueryParams) -> Json<endpoint_models::Table> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let table = table::get_table(&mut establish_connection(), id, &expands).unwrap();
    Json::from(table)
}

/// Get all tables.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of tables.
///
/// # Example
///
/// ```
/// GET /table?expands=entity1,entity2
/// ```
#[openapi]
#[get("/table?<query..>")]
fn get_all_tables(query: QueryParams) -> Json<Vec<endpoint_models::Table>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let tables = table::get_all_tables(&mut establish_connection(), &expands).unwrap();
    Json::from(tables)
}

/// Get a reservation by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the reservation.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the reservation.
///
/// # Example
///
/// ```
/// GET /reservation/1?expands=entity1,entity2
/// ```
#[openapi]
#[get("/reservation/<id>?<query..>")]
fn get_reservation(id: i32, query: QueryParams) -> Json<endpoint_models::Reservation> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let reservation =
        reservation::get_reservation(&mut establish_connection(), id, &expands).unwrap();
    Json::from(reservation)
}

/// Get all reservations.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of reservations.
///
/// # Example
///
/// ```
/// GET /reservation?expands=entity1,entity2
/// ```
#[openapi]
#[get("/reservation?<query..>")]
fn get_all_reservations(query: QueryParams) -> Json<Vec<endpoint_models::Reservation>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let reservations =
        reservation::get_all_reservations(&mut establish_connection(), &expands).unwrap();
    Json::from(reservations)
}

/// Get a person by ID.
///
/// # Path Parameters
///
/// - `id`: The ID of the person.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns the person.
///
/// # Example
///
/// ```
/// GET /person/1?expands=entity1,entity2
/// ```
#[openapi]
#[get("/person/<id>?<query..>")]
fn get_person(id: i32, query: QueryParams) -> Json<endpoint_models::Person> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    println!("Expansions: {}", expands.join(", "));

    let person = person::get_person(&mut establish_connection(), id, &expands).unwrap();
    Json::from(person)
}

/// Get all persons.
///
/// # Query Parameters
///
/// - `expands`: Comma-separated list of related entities to expand.
///
/// # Returns
///
/// - `200 OK`: Returns a list of persons.
///
/// # Example
///
/// ```
/// GET /person?expands=entity1,entity2
/// ```
#[openapi]
#[get("/person?<query..>")]
fn get_all_persons(query: QueryParams) -> Json<Vec<endpoint_models::Person>> {
    let expands: Vec<&str> = query
        .expands
        .as_ref()
        .map(|s| s.iter().flat_map(|v| v.split(',')).collect())
        .unwrap_or_default();

    let persons = person::get_all_persons(&mut establish_connection(), &expands).unwrap();
    Json::from(persons)
}

/// Insert a new person.
///
/// # Request Body
///
/// - `person`: The person data to insert.
///
/// # Returns
///
/// - `200 OK`: Returns the inserted person.
///
/// # Example
///
/// ```
/// POST /person
/// {
///     "name": "John Doe",
///     "phone": "john.doe@example.com"
/// }
/// ```
#[openapi]
#[post("/person", format = "json", data = "<person>")]
fn post_person(person: Json<endpoint_models::Person>) -> Json<endpoint_models::Person> {
    let response = person::insert_person(&mut establish_connection(), person);
    Json(response)
}

/// Insert a new reservation.
///
/// # Request Body
///
/// - `reservation`: The reservation data to insert.
///
/// # Returns
///
/// - `200 OK`: Returns the inserted reservation.
///
/// # Example
///
/// ```
/// POST /reservation
/// {
///  "id_person": 0,
///  "start_timestamp": "string",
///  "end_timestamp": "string",
///  "person_count": 0
/// }
/// ```
#[openapi]
#[post("/reservation", format = "json", data = "<reservation>")]
fn post_reservation(
    reservation: Json<endpoint_models::Reservation>,
) -> Json<endpoint_models::Reservation> {
    let response = reservation::insert_reservation(&mut establish_connection(), reservation);
    Json(response)
}

#[openapi]
#[post("/table", format = "json", data = "<table>")]
fn post_table(table: Json<endpoint_models::Table>) -> Json<endpoint_models::Table> {
    let response = table::insert_table(&mut establish_connection(), table);
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
                get_all_tables,
                get_reservation,
                get_all_reservations,
                get_person,
                get_all_persons,
                post_person,
                post_reservation,
                post_table
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
