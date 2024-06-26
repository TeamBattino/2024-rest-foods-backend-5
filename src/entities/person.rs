//! This module provides functionality for retrieving persons and their related data from the database.
//!
//! # Functions
//! - `get_person`: Retrieves a single person by its ID, including optional expansions for reservations.
//! - `get_all_persons`: Retrieves all persons, including optional expansions for reservations.
//! - `expand_reservations`: Helper function to expand reservations for a person.

use diesel::{dsl::insert_into, ExpressionMethods, PgConnection, RunQueryDsl};
use rocket::serde::json::Json;

use crate::models;

use crate::endpoint_models;
use crate::schema::person;
use crate::schema::reservation::{self};
use diesel::prelude::*;
use diesel::result::Error;

use super::reservation::get_reservation;

/// Retrieves a single person by its ID, including optional expansions for reservations.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `id` - The ID of the person to retrieve.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing the person endpoint model or a Diesel error.
pub fn get_person(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Person, Error> {
    let models_person: models::Person = person::dsl::person
        .find(id)
        .select(models::Person::as_select())
        .first::<models::Person>(conn)?;

    // expand reservations
    let reservations = expand_reservations(conn, id, expansions);

    let endpoints_person: endpoint_models::Person = endpoint_models::Person {
        person_id: models_person.person_id,
        name: models_person.name,
        phone: models_person.phone,
        reservations: reservations,
    };
    Ok(endpoints_person)
}

/// Retrieves all persons, including optional expansions for reservations.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing a vector of person endpoint models or a Diesel error.
pub fn get_all_persons(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Person>, Error> {
    let models_persons = person::dsl::person
        .select(models::Person::as_select())
        .load::<models::Person>(conn)?;
    let entrypoints_persons = models_persons
        .iter()
        .map(|p| {
            // expand reservations
            let reservations = expand_reservations(conn, p.person_id, expansions);

            let endpoints_person: endpoint_models::Person = endpoint_models::Person {
                person_id: p.person_id,
                name: p.name.clone(),
                phone: p.phone.clone(),
                reservations: reservations,
            };
            endpoints_person
        })
        .collect();
    Ok(entrypoints_persons)
}

/*   Expansions   */

/// Helper function to expand reservations for a person.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `person_id` - The ID of the person for which to expand reservations.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of reservation endpoint models.
fn expand_reservations(
    conn: &mut PgConnection,
    person_id: i32,
    expansions: &Vec<&str>,
) -> Option<Vec<endpoint_models::Reservation>> {
    if !expansions.contains(&"reservations") {
        return None;
    }

    let reservation_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("reservations."))
        .map(|e| &e[13..])
        .collect();

    // Get reservations
    let relations: Vec<models::Reservation> = reservation::dsl::reservation
        .filter(reservation::dsl::id_person.eq(person_id))
        .select(models::Reservation::as_select())
        .load::<models::Reservation>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_reservation(conn, rel.reservation_id, &reservation_expansions).unwrap())
            .collect(),
    )
}

pub fn insert_person(
    conn: &mut PgConnection,
    person: Json<models::Person>,
) -> models::Person {
    insert_into(person::dsl::person)
        .values((
            person::name.eq(person.name.clone()),
            person::phone.eq(person.phone.clone()),
        ))
        .get_result(conn)
        .unwrap()
}
