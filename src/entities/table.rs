//! This module provides functionality for retrieving tables and their related data from the database.
//!
//! # Functions
//! - `get_table`: Retrieves a single table by its ID, including optional expansions for reservations.
//! - `get_all_tables`: Retrieves all tables, including optional expansions for reservations.
//! - `expand_reservations`: Helper function to expand reservations for a table.

use crate::schema::table::{coord_x, coord_y, height, seat_count, width};
use crate::schema::{self, table_reservation};
use crate::{endpoint_models, models};
use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;
use rocket::serde::json::Json;

use super::reservation::get_reservation;

/// Retrieves a single table by its ID, including optional expansions for reservations.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `id` - The ID of the table to retrieve.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing the table endpoint model or a Diesel error.
pub fn get_table(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Table, Error> {
    let models_table: models::Table = schema::table::dsl::table
        .find(id)
        .select(models::Table::as_select())
        .first::<models::Table>(conn)?;

    // expand reservations
    let reservations = expand_reservations(conn, id, expansions);

    let endpoints_table: endpoint_models::Table = endpoint_models::Table {
        table_id: models_table.table_id,
        seat_count: models_table.seat_count,
        coord_x: models_table.coord_x,
        coord_y: models_table.coord_y,
        width: models_table.width,
        height: models_table.height,
        reservations: reservations,
    };
    Ok(endpoints_table)
}

/// Retrieves all tables, including optional expansions for reservations.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing a vector of table endpoint models or a Diesel error.
pub fn get_all_tables(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Table>, Error> {
    let models_tables = schema::table::dsl::table
        .select(models::Table::as_select())
        .load::<models::Table>(conn)?;
    let entrypoints_tables = models_tables
        .iter()
        .map(|t| {
            // expand reservations
            let reservations = expand_reservations(conn, t.table_id, expansions);

            let endpoints_table: endpoint_models::Table = endpoint_models::Table {
                table_id: t.table_id,
                seat_count: t.seat_count,
                coord_x: t.coord_x,
                coord_y: t.coord_y,
                width: t.width,
                height: t.height,
                reservations: reservations,
            };
            endpoints_table
        })
        .collect();
    Ok(entrypoints_tables)
}

/*   Expansions   */

/// Helper function to expand reservations for a table.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `table_id` - The ID of the table for which to expand reservations.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of reservation endpoint models.
fn expand_reservations(
    conn: &mut PgConnection,
    table_id: i32,
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
    let relations: Vec<models::TableReservation> = table_reservation::dsl::table_reservation
        .filter(table_reservation::dsl::id_table.eq(table_id))
        .select(models::TableReservation::as_select())
        .load::<models::TableReservation>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_reservation(conn, rel.id_reservation, &reservation_expansions).unwrap())
            .collect(),
    )
}

pub fn insert_table(
    conn: &mut PgConnection,
    table: Json<endpoint_models::Table>
) -> endpoint_models::Table{
    let models_table: models::Table = insert_into(schema::table::dsl::table).values((
        seat_count.eq(table.seat_count),
        coord_x.eq(table.coord_x),
        coord_y.eq(table.coord_y),
        width.eq(table.width),
        height.eq(table.height)
    ))
    .get_result(conn).unwrap();

    endpoint_models::Table {
        table_id: models_table.table_id,
        seat_count: models_table.seat_count,
        coord_x: models_table.coord_x,
        coord_y: models_table.coord_y,
        width: models_table.width,
        height: models_table.height,
        reservations: None
    }
}
