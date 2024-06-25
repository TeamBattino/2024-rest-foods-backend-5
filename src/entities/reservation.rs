use diesel::{dsl::insert_into, ExpressionMethods, PgConnection, RunQueryDsl};
use rocket::serde::json::Json;

use crate::{
    db::establish_connection,
    inserteable_models, models,
    schema::{self, reservation},
};

pub fn insert_reservation(
    conn: &mut PgConnection,
    reservation: Json<inserteable_models::Reservation>,
) -> models::Reservation {
    insert_into(schema::reservation::dsl::reservation)
        .values((
            reservation::id_person.eq(reservation.id_person.clone()),
            reservation::start_timestamp.eq(reservation.start_timestamp.clone()),
            reservation::end_timestamp.eq(reservation.end_timestamp.clone()),
            reservation::person_count.eq(reservation.person_count.clone()),
        ))
        .get_result(conn)
        .unwrap()
}
