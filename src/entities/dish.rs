use crate::schema::dish::name;
use crate::schema::menucard::{self, menucard_id};
use crate::schema::{dish, dish_tag, tag};
use crate::{endpoint_models, models};
use diesel::dsl::select;
use diesel::expression::is_aggregate::No;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::tags::get_tag;

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

    // TODO: Menucard expansions

    let endpoints_dish: endpoint_models::Dish = endpoint_models::Dish {
        dish_id: models_dish.dish_id,
        chefs_choice: chefs_choice,
        description: models_dish.description,
        name: models_dish.name,
        dish_type: models_dish.dish_type,
        menucards: None,
        tags: tags,
    };
    Ok(endpoints_dish)
}

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

            // TODO: Menucard expansions

            let endpoints_dish: endpoint_models::Dish = endpoint_models::Dish {
                dish_id: d.dish_id,
                chefs_choice: None,
                description: d.description.clone(),
                name: d.name.clone(),
                dish_type: d.dish_type.clone(),
                menucards: None,
                tags: tags,
            };
            endpoints_dish
        })
        .collect();
    Ok(entrypoints_dishes)
}

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
