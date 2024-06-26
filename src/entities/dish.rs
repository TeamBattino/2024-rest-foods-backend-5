//! This module provides functionality for retrieving dishes and their related data from the database.
//!
//! # Functions
//! - `get_dish`: Retrieves a single dish by its ID, including optional expansions for tags and menucards.
//! - `get_all_dishes`: Retrieves all dishes, including optional expansions for tags and menucards.
//! - `expand_tags`: Helper function to expand tags for a dish.
//! - `expand_menucards`: Helper function to expand menucards for a dish.

use crate::schema::{dish, dish_tag, menucard_dish};
use crate::{endpoint_models, models};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::menucard::get_menucard;
use super::tag::get_tag;

/// Retrieves a single dish by its ID, including optional expansions for tags and menucards.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `id` - The ID of the dish to retrieve.
/// * `expansions` - A vector of strings specifying which related data to expand.
/// * `chefs_choice` - An optional boolean to indicate if the dish is a chef's choice.
///
/// # Returns
///
/// A result containing the dish endpoint model or a Diesel error.
pub fn get_dish(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
    chefs_choice: Option<bool>,
) -> Result<endpoint_models::Dish, Error> {
    let models_dish: models::Dish = dish::dsl::dish
        .find(id)
        .select(models::Dish::as_select())
        .first::<models::Dish>(conn)?;
    // expand tags
    let tags = expand_tags(conn, id, expansions);

    // Menucard expansions
    let menucards = expand_menucards(conn, id, expansions);

    let endpoints_dish: endpoint_models::Dish = endpoint_models::Dish {
        dish_id: models_dish.dish_id,
        chefs_choice: chefs_choice,
        description: models_dish.description,
        name: models_dish.name,
        dish_type: models_dish.dish_type,
        menucards: menucards,
        tags: tags,
    };
    Ok(endpoints_dish)
}

/// Retrieves all dishes, including optional expansions for tags and menucards.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing a vector of dish endpoint models or a Diesel error.
pub fn get_all_dishes(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Dish>, Error> {
    let models_dishes = dish::dsl::dish
        .select(models::Dish::as_select())
        .load::<models::Dish>(conn)?;
    let entrypoints_dishes = models_dishes
        .iter()
        .map(|d| {
            // expand dishes
            let tags = expand_tags(conn, d.dish_id, expansions);

            // Menucard expansions
            let menucards = expand_menucards(conn, d.dish_id, expansions);

            let endpoints_dish: endpoint_models::Dish = endpoint_models::Dish {
                dish_id: d.dish_id,
                chefs_choice: None,
                description: d.description.clone(),
                name: d.name.clone(),
                dish_type: d.dish_type.clone(),
                menucards: menucards,
                tags: tags,
            };
            endpoints_dish
        })
        .collect();
    Ok(entrypoints_dishes)
}

/// Helper function to expand tags for a dish.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `dish_id` - The ID of the dish for which to expand tags.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of tag endpoint models.
fn expand_tags(
    conn: &mut PgConnection,
    dish_id: i32,
    expansions: &Vec<&str>,
) -> Option<Vec<endpoint_models::Tag>> {
    if !expansions.contains(&"tags") {
        return None;
    }

    let tag_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("tags."))
        .map(|e| &e[5..])
        .collect();

    // Get relations
    let relations: Vec<models::DishTag> = dish_tag::dsl::dish_tag
        .filter(dish_tag::dsl::id_dish.eq(dish_id))
        .select(models::DishTag::as_select())
        .load::<models::DishTag>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_tag(conn, rel.id_tag, &tag_expansions).unwrap())
            .collect(),
    )
}

/// Helper function to expand menucards for a dish.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `dish_id` - The ID of the dish for which to expand menucards.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing a vector of menucard endpoint models.
fn expand_menucards(
    conn: &mut PgConnection,
    dish_id: i32,
    expansions: &Vec<&str>,
) -> Option<Vec<endpoint_models::Menucard>> {
    if !expansions.contains(&"menucards") {
        return None;
    }

    let menucards_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("menucards."))
        .map(|e| &e[10..])
        .collect();

    // Get relations
    let relations: Vec<models::MenucardDish> = menucard_dish::dsl::menucard_dish
        .filter(menucard_dish::dsl::id_dish.eq(dish_id))
        .select(models::MenucardDish::as_select())
        .load::<models::MenucardDish>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| get_menucard(conn, rel.id_menucard, &menucards_expansions).unwrap())
            .collect(),
    )
}
