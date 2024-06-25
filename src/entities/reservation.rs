//! This module provides functionality for retrieving reservations and their related data from the database.
//!
//! # Functions
//! - `get_reservation`: Retrieves a single reservation by its ID, including optional expansions for tables and person.
//! - `get_all_reservations`: Retrieves all reservations, including optional expansions for tables and person.
//! - `expand_tables`: Helper function to expand tables for a reservation.
//! - `expand_person`: Helper function to expand person for a reservation.

use diesel::{dsl::insert_into, ExpressionMethods, PgConnection, RunQueryDsl};
use rocket::serde::json::Json;

use crate::inserteable_models;
use crate::schema::table::{self};
use crate::schema::{reservation, table_reservation};
use crate::{endpoint_models, models};

use diesel::prelude::*;
use diesel::result::Error;

use super::person::get_person;
use super::table::get_table;

/// Retrieves a single reservation by its ID, including optional expansions for tables and person.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `id` - The ID of the reservation to retrieve.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing the reservation endpoint model or a Diesel error.
pub fn get_reservation(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Reservation, Error> {
    let models_reservation: models::Reservation = reservation::dsl::reservation
        .find(id)
        .select(models::Reservation::as_select())
        .first::<models::Reservation>(conn)?;

    // expand tables and person
    let tables = expand_tables(conn, id, expansions);
    let person = expand_person(conn, models_reservation.id_person, expansions);

    let endpoints_reservation: endpoint_models::Reservation = endpoint_models::Reservation {
        reservation_id: models_reservation.reservation_id,
        id_person: models_reservation.id_person,
        start_timestamp: models_reservation.start_timestamp,
        end_timestamp: models_reservation.end_timestamp,
        person_count: models_reservation.person_count,
        tables: tables,
        person: person,
    };
    Ok(endpoints_reservation)
}

/// Retrieves all reservations, including optional expansions for tables and person.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing a vector of reservation endpoint models or a Diesel error.
pub fn get_all_reservations(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Reservation>, Error> {
    let models_reservations = reservation::dsl::reservation
        .select(models::Reservation::as_select())
        .load::<models::Reservation>(conn)?;
    let entrypoints_reservations = models_reservations
        .iter()
        .map(|r| {
            // expand tables and person
            let tables = expand_tables(conn, r.reservation_id, expansions);
            let person = expand_person(conn, r.id_person, expansions);

            let endpoints_reservation: endpoint_models::Reservation =
                endpoint_models::Reservation {
                    reservation_id: r.reservation_id,
                    id_person: r.id_person,
                    start_timestamp: r.start_timestamp,
                    end_timestamp: r.end_timestamp,
                    person_count: r.person_count,
                    tables: tables,
                    person: person,
                };
            endpoints_reservation
        })
        .collect();
    Ok(entrypoints_reservations)
}

/*   Expansions   */

/// Helper function to expand tables for a reservation.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `reservation_id` - The ID of the reservation for which to expand tables.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of table endpoint models.
fn expand_tables(
    conn: &mut PgConnection,
    reservation_id: i32,
    expansions: &Vec<&str>,
) -> Option<Vec<endpoint_models::Table>> {
    if !expansions.contains(&"tables") {
        return None;
    }

    let table_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("tables."))
        .map(|e| &e[7..])
        .collect();

    // Get tables
    let relations: Vec<models::TableReservation> = table_reservation::dsl::table_reservation
        .filter(table_reservation::dsl::id_reservation.eq(reservation_id))
        .select(models::TableReservation::as_select())
        .load::<models::TableReservation>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_table(conn, rel.id_table, &table_expansions).unwrap())
            .collect(),
    )
}

/// Helper function to expand person for a reservation.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `person_id` - The ID of the person for which to expand.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing the person endpoint model.
fn expand_person(
    conn: &mut PgConnection,
    person_id: i32,
    expansions: &Vec<&str>,
) -> Option<endpoint_models::Person> {
    if !expansions.contains(&"person") {
        return None;
    }

    get_person(conn, person_id, expansions).ok()
}

pub fn insert_reservation(
    conn: &mut PgConnection,
    reservation: Json<inserteable_models::Reservation>,
) -> models::Reservation {
    insert_into(reservation::dsl::reservation)
        .values((
            reservation::id_person.eq(reservation.id_person.clone()),
            reservation::start_timestamp.eq(reservation.start_timestamp.clone()),
            reservation::end_timestamp.eq(reservation.end_timestamp.clone()),
            reservation::person_count.eq(reservation.person_count.clone()),
        ))
        .get_result(conn)
        .unwrap()
}
