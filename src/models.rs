use diesel::prelude::*;

use crate::json_date;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::dish)]
pub struct Dish {
    pub dish_id: i32,
    pub name: String,
    pub description: String,
    pub dish_type: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tag)]
pub struct Tag {
    pub tag_id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::dish_tag)]
pub struct DishTag {
    pub dish_tag_id: i32,
    pub id_dish: i32,
    pub id_tag: i32,
}

#[derive(Queryable, Selectable, Insertable, Identifiable)]
#[diesel(table_name = crate::schema::menucard)]
#[primary_key(menucard_id)]
pub struct Menucard {
    pub menucard_id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::menucard_dish)]
pub struct MenucardDish {
    pub menucard_dish_id: i32,
    pub id_menucard: i32,
    pub id_dish: i32,
    pub chefs_choice: bool,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::setting)]
pub struct Setting {
    pub setting_id: i32,
    pub id_menucard_active: i32,
    pub restaurant_width: i32,
    pub restaurant_height: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::table)]
pub struct Table {
    pub table_id: i32,
    pub seat_count: i32,
    pub coord_x: i32,
    pub coord_y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Queryable, Selectable, Deserialize, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::reservation)]
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
}

#[derive(Queryable, Selectable, Deserialize, Serialize, JsonSchema)]
#[diesel(table_name = crate::schema::person)]
pub struct Person {
    pub person_id: i32,
    pub name: String,
    pub phone: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::table_reservation)]
pub struct TableReservation {
    pub table_reservation_id: i32,
    pub id_table: i32,
    pub id_reservation: i32,
}
