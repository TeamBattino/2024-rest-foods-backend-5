use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::json_date;

#[derive(Deserialize)]
pub struct Person {
    pub name: String,
    pub phone: String,
}



#[derive(Deserialize)]
pub struct Reservation {
    pub id_person: i32,
    #[serde(with = "json_date")]
    pub start_timestamp: NaiveDateTime,
    #[serde(with = "json_date")]
    pub end_timestamp: NaiveDateTime,
    pub person_count: i32,
}