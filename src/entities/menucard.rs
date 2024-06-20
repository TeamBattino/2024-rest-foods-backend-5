use crate::schema::menucard::{self, menucard_id};
use crate::schema::menucard_dish::{self, chefs_choice};
use crate::{endpoint_models, models, Dish, MenucardDish};
use diesel::result::Error;
use diesel::PgConnection;
use diesel::{prelude::*, result};

use super::dish::get_dish;

pub fn get_menucard(
    conn: &mut PgConnection,
    id: i32,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Menucard, Error> {
    let models_menucard: models::Menucard = menucard::dsl::menucard
        .find(id)
        .select(models::Menucard::as_select())
        .first::<models::Menucard>(conn)?;

    // expand dishes
    let mut dishes = None;
    if expansions.contains(&"dishes") {
        let dish_expansions: Vec<&str> = expansions
            .iter()
            .filter(|e| e.starts_with("dishes."))
            .map(|e| &e[7..])
            .collect();

        // Get relations
        let relations: Vec<MenucardDish> = menucard_dish::dsl::menucard_dish
            .filter(menucard_dish::dsl::id_menucard.eq(id))
            .select(models::MenucardDish::as_select())
            .load::<models::MenucardDish>(conn)?;

        dishes = Some(
            relations
                .iter()
                .map(|rel| {
                    get_dish(conn, rel.id_dish, &dish_expansions, Some(rel.chefs_choice)).unwrap()
                })
                .collect(),
        );
    }

    let endpoints_menucard: endpoint_models::Menucard = endpoint_models::Menucard {
        menucard_id: models_menucard.menucard_id,
        name: models_menucard.name,
        dishes: dishes,
    };
    Ok(endpoints_menucard)
}

pub fn get_all_menucards(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
    id: i32,
) -> Result<Vec<endpoint_models::Menucard>, Error> {
    let models_menucards = menucard::dsl::menucard
        .select(models::Menucard::as_select())
        .load::<models::Menucard>(conn)?;
    let entrypoints_menucards = models_menucards
        .iter()
        .map(|mc| {
            // expand dishes
            let mut dishes = None;
            if expansions.contains(&"dishes") {
                let dish_expansions: Vec<&str> = expansions
                    .iter()
                    .filter(|e| e.starts_with("dishes."))
                    .map(|e| &e[7..])
                    .collect();

                // Get relations
                let relations: Vec<MenucardDish> = menucard_dish::dsl::menucard_dish
                    .filter(menucard_dish::dsl::id_menucard.eq(id))
                    .select(models::MenucardDish::as_select())
                    .load::<models::MenucardDish>(conn)
                    .unwrap();

                dishes = Some(
                    relations
                        .iter()
                        .map(|rel| {
                            get_dish(conn, rel.id_dish, &dish_expansions, Some(rel.chefs_choice))
                                .unwrap()
                        })
                        .collect(),
                );
            }

            let endpoints_menucard: endpoint_models::Menucard = endpoint_models::Menucard {
                menucard_id: mc.menucard_id,
                name: mc.name.clone(),
                dishes: dishes,
            };
            endpoints_menucard
        })
        .collect();
    Ok(entrypoints_menucards)
}

/*
pub fn get_menucard(
    conn: &mut PgConnection,
    expansions: Vec<Vec<&str>>,
    id: IdType,
) -> Result<Vec<endpoint_models::Menucard>, Error> {
    let model_menucards: Vec<models::Menucard> = match id {
        IdType::Single(single_id) => {
            let menucard = menucard::dsl::menucard
                .find(single_id)
                .select(models::Menucard::as_select())
                .first::<models::Menucard>(conn)?;
            vec![menucard]
        }
        IdType::Multiple(ids) => menucard::dsl::menucard
            .filter(menucard::dsl::menucard_id.eq_any(ids))
            .select(models::Menucard::as_select())
            .load::<models::Menucard>(conn)?,
        IdType::All => menucard::dsl::menucard
            .select(models::Menucard::as_select())
            .load::<models::Menucard>(conn)?,
    };
    let mut endpoint_menucards: Vec<endpoint_models::Menucard> = model_menucards
        .into_iter()
        .map(|mc| endpoint_models::Menucard {
            menucard_id: mc.menucard_id,
            name: mc.name,
            dishes: None,
        })
        .collect();

    if expansions.len() > 1 {
        let current_layer_expansions = &expansions[0];
        let next_layer_expansions = &expansions[1..];
        for current in &model_menucards {
            for expansion in current_layer_expansions {
                // Implement the expansion logic here
                todo!()
            }
        }
    }
    Ok(endpoint_menucards)
}
 */
