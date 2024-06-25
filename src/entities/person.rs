use diesel::{dsl::insert_into, ExpressionMethods, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{db::establish_connection, inserteable_models, models, schema::{self, person::{name, phone}}};

pub fn insert_person(person: Json<inserteable_models::Person>) -> models::Person{
    insert_into(schema::person::dsl::person).values((
        name.eq(person.name.clone()), 
        phone.eq(person.phone.clone()
    ))).get_result(&mut establish_connection()).unwrap()
}