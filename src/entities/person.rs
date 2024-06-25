use diesel::{dsl::insert_into, ExpressionMethods, PgConnection, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{
    db::establish_connection,
    inserteable_models, models,
    schema::{self, person},
};

pub fn insert_person(
    conn: &mut PgConnection,
    person: Json<inserteable_models::Person>,
) -> models::Person {
    insert_into(schema::person::dsl::person)
        .values((
            person::name.eq(person.name.clone()),
            person::phone.eq(person.phone.clone()),
        ))
        .get_result(conn)
        .unwrap()
}
