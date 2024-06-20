use crate::schema::menucard;
use crate::schema::menucard_dish::{self, chefs_choice};
use crate::{endpoint_models, models};
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
    let dishes = expand_dishes(conn, id, expansions);

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
) -> Result<Vec<endpoint_models::Menucard>, Error> {
    let models_menucards = menucard::dsl::menucard
        .select(models::Menucard::as_select())
        .load::<models::Menucard>(conn)?;
    let entrypoints_menucards = models_menucards
        .iter()
        .map(|mc| {
            // expand dishes
            let dishes = expand_dishes(conn, mc.menucard_id, expansions);

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

/*   Expansions   */

fn expand_dishes(
    conn: &mut PgConnection,
    menucard_id: i32,
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
    let relations: Vec<models::MenucardDish> = menucard_dish::dsl::menucard_dish
        .filter(menucard_dish::dsl::id_menucard.eq(menucard_id))
        .select(models::MenucardDish::as_select())
        .load::<models::MenucardDish>(conn)
        .unwrap();

    Some(
        relations
            .iter()
            .map(|rel| {
                get_dish(conn, rel.id_dish, &dish_expansions, Some(rel.chefs_choice)).unwrap()
            })
            .collect(),
    )
}
