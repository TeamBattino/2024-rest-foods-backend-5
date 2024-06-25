use crate::schema::{dish_tag, tag};
use crate::{endpoint_models, models};
use diesel::prelude::*;
use diesel::result::Error;
use diesel::PgConnection;

use super::dish::get_dish;

pub fn get_tag(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Tag, Error> {
    let models_tag: models::Tag = tag::dsl::tag
        .find(id)
        .select(models::Tag::as_select())
        .first::<models::Tag>(conn)?;
    // expand Dishes
    let dishes = expand_dishes(conn, id, expansions);
    let endpoints_tag: endpoint_models::Tag = endpoint_models::Tag {
        name: models_tag.name,
        dishes: dishes,
        tag_id: models_tag.tag_id,
    };
    Ok(endpoints_tag)
}

pub fn get_all_tags(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<Vec<endpoint_models::Tag>, Error> {
    let models_dishes = tag::dsl::tag
        .select(models::Tag::as_select())
        .load::<models::Tag>(conn)?;
    let entrypoints_dishes = models_dishes
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
    Ok(entrypoints_dishes)
}

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
