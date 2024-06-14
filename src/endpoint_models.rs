use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::json_date;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub description: String,
    pub dish_type: String,
    pub tags: Option<Vec<Tag>>,
    pub chefs_choice: Option<bool>,
    pub menucards: Option<Vec<Menucard>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub tag_id: i32,
    pub name: String,
    pub dishes: Option<Vec<Dish>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Menucard {
    pub menucard_id: i32,
    pub name: String,
    pub dishes: Option<Vec<Dish>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    pub setting_id: i32,
    pub id_menucard_active: i32,
    pub restaurant_width: i32,
    pub restaurant_height: i32,
    pub menucard_active: Option<Menucard>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub table_id: i32,
    pub seat_count: i32,
    pub coord_x: i32,
    pub coord_y: i32,
    pub width: i32,
    pub height: i32,
    pub reservations: Option<Vec<Reservation>>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Reservation {
    pub reservation_id: i32,
    pub id_person: i32,
    #[serde(with = "json_date")]
    pub start_timestamp: NaiveDateTime,
    #[serde(with = "json_date")]
    pub end_timestamp: NaiveDateTime,
    pub person_count: i32,
    pub tables: Option<Vec<Table>>,
    pub person: Option<Person>,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub person_id: i32,
    pub name: String,
    pub phone: String,
    pub reservations: Option<Vec<Reservation>>,
}
