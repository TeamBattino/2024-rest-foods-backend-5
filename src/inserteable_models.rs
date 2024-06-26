use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::json_date;

#[derive(Deserialize, JsonSchema)]
pub struct Person {
    pub name: String,
    pub phone: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct Reservation {
    pub id_person: i32,
    #[serde(deserialize_with = "json_date::deserialize")]
    pub start_timestamp: NaiveDateTime,
    #[serde(deserialize_with = "json_date::deserialize")]
    pub end_timestamp: NaiveDateTime,
    pub person_count: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct Dish {
    pub name: String,
    pub description: String,
    pub dish_type: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct Tag {
    pub name: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct DishTag {
    pub id_dish: i32,
    pub id_tag: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct MenucardDish {
    pub id_menucard: i32,
    pub id_dish: i32,
    pub chefs_choice: bool,
}

#[derive(Deserialize, JsonSchema)]
pub struct Table {
    pub seat_count: i32,
    pub coord_x: i32,
    pub coord_y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize, JsonSchema)]
pub struct TableReservation {
    pub id_table: i32,
    pub id_reservation: i32,
}
