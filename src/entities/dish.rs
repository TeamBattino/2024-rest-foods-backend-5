use crate::schema::dish::name;
use crate::schema::menucard::{self, menucard_id};
use crate::schema::{dish, dish_tag, tag};
use crate::{endpoint_models, models, DishTag};
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
    let mut tags: Option<Vec<endpoint_models::Tag>> = None;
    if expansions.contains(&"tags") {
        let tag_expansions: Vec<&str> = expansions
            .iter()
            .filter(|e| e.starts_with("tags."))
            .map(|e| &e[5..])
            .collect();

        // Get relations
        let relations: Vec<DishTag> = dish_tag::dsl::dish_tag
            .filter(dish_tag::dsl::id_dish.eq(id))
            .select(models::DishTag::as_select())
            .load::<models::DishTag>(conn)?;
        tags = Some(
            relations
                .iter()
                .map(|rel| get_tag(conn, rel.id_tag, &tag_expansions).unwrap())
                .collect(),
        );
    }
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
