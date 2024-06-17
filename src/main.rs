use std::collections::HashSet;

use db::{establish_connection, get_db_settings};
use diesel::PgConnection;

use models::*;
mod db;
mod endpoint_models;
mod entities;
mod json_date;
mod models;
mod rocket;
mod schema;

fn main() {
    /* let mut db_connection = establish_connection();
    let settings = get_db_settings(&mut db_connection);
    entities::menucard::get_menucard(&mut db_connection, 1, vec!["dishes"]) */
    expansions_to_layered(vec![
        "dish.hello.yes",
        "dish.hello.no",
        "epic.hello.bro",
        "dish",
        "dish.yes",
    ]);
}

fn expansions_to_layered(expansions: Vec<&str>) -> Vec<Vec<&str>> {
    // Split expansions by dots into a vector of vectors
    let expasions_split_by_dot: Vec<_> = expansions
        .iter()
        .map(|expansion| expansion.split(".").collect::<Vec<&str>>())
        .collect();
    let num_rows = expasions_split_by_dot.len();
    let num_cols = expasions_split_by_dot.get(0).map_or(0, |row| row.len());

    let mut layered_vector: Vec<Vec<&str>> = vec![Vec::with_capacity(num_rows); num_cols];

    for (i, row) in expasions_split_by_dot.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            layered_vector[j].push(col);
        }
    }
    for vec in &mut layered_vector {
        let mut seen = HashSet::new();
        vec.retain(|&x| seen.insert(x) && !x.is_empty());
    }
    return layered_vector;
}
