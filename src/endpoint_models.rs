use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::json_date;

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub description: String,
    pub dish_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chefs_choice: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menucards: Option<Vec<Menucard>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Tag {
    pub tag_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dishes: Option<Vec<Dish>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Menucard {
    pub menucard_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dishes: Option<Vec<Dish>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Setting {
    pub setting_id: i32,
    pub id_menucard_active: i32,
    pub restaurant_width: i32,
    pub restaurant_height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menucard_active: Option<Menucard>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Table {
    pub table_id: i32,
    pub seat_count: i32,
    pub coord_x: i32,
    pub coord_y: i32,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Reservation {
    pub reservation_id: i32,
    pub id_person: i32,
    #[serde(deserialize_with = "json_date::deserialize")]
    #[serde(serialize_with = "json_date::serialize")]
    pub start_timestamp: NaiveDateTime,
    #[serde(deserialize_with = "json_date::deserialize")]
    #[serde(serialize_with = "json_date::serialize")]
    pub end_timestamp: NaiveDateTime,
    pub person_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables: Option<Vec<Table>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Person {
    pub person_id: i32,
    pub name: String,
    pub phone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}
