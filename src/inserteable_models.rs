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