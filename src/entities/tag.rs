//! This module provides functionality for retrieving tags and their related data from the database.
//!
//! # Functions
//! - `get_tag`: Retrieves a single tag by its ID, including optional expansions for dishes.
//! - `get_all_tags`: Retrieves all tags, including optional expansions for dishes.
//! - `expand_dishes`: Helper function to expand dishes for a tag.

use crate::schema::{dish_tag, tag};
use crate::{endpoint_models, models};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::dish::get_dish;

/// Retrieves a single tag by its ID, including optional expansions for dishes.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `id` - The ID of the tag to retrieve.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing the tag endpoint model or a Diesel error.
pub fn get_tag(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Tag, Error> {
    let models_tag: models::Tag = tag::dsl::tag
        .find(id)
        .select(models::Tag::as_select())
        .first::<models::Tag>(conn)?;
    // expand dishes
    let dishes = expand_dishes(conn, id, expansions);
    let endpoints_tag: endpoint_models::Tag = endpoint_models::Tag {
        name: models_tag.name,
        dishes: dishes,
        tag_id: models_tag.tag_id,
    };
    Ok(endpoints_tag)
}

/// Retrieves all tags, including optional expansions for dishes.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing a vector of tag endpoint models or a Diesel error.
pub fn get_all_tags(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Tag>, Error> {
    let models_tags = tag::dsl::tag
        .select(models::Tag::as_select())
        .load::<models::Tag>(conn)?;
    let entrypoints_tags = models_tags
        .iter()
        .map(|t| {
            // expand dishes
            let dishes = expand_dishes(conn, t.tag_id, expansions);

            let endpoints_tag: endpoint_models::Tag = endpoint_models::Tag {
                name: t.name.clone(),
                dishes: dishes,
                tag_id: t.tag_id,
            };
            endpoints_tag
        })
        .collect();
    Ok(entrypoints_tags)
}

/// Helper function to expand dishes for a tag.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `tag_id` - The ID of the tag for which to expand dishes.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of dish endpoint models.
fn expand_dishes(
    conn: &mut PgConnection,
    tag_id: i32,
    expansions: &Vec<&str>,
) -> Option<Vec<endpoint_models::Dish>> {
    if !expansions.contains(&"dishes") {
        return None;
    }

    let dish_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("dishes."))
        .map(|e| &e[7..])
        .collect();

    // Get relations
    let relations: Vec<models::DishTag> = dish_tag::dsl::dish_tag
        .filter(dish_tag::dsl::id_tag.eq(tag_id))
        .select(models::DishTag::as_select())
        .load::<models::DishTag>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_dish(conn, rel.id_dish, &dish_expansions, None).unwrap())
            .collect(),
    )
}
