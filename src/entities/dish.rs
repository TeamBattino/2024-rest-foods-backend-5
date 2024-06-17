use crate::schema::menucard::{self, menucard_id};
use crate::{endpoint_models, models};
use diesel::dsl::select;
use diesel::prelude::*;
use diesel::PgConnection;

pub fn get_dish(conn: &mut PgConnection, id: i32, expansions: Vec<Vec<&str>>) {
    let result: models::Menucard = menucard::dsl::menucard
        .find(id)
        .select(models::Menucard::as_select())
        .first(conn)
        .unwrap();
    let mut api_object: endpoint_models::Menucard = endpoint_models::Menucard {
        dishes: None,
        menucard_id: result.menucard_id,
        name: result.name,
    };

    if expansions.len() > 1 {
        expansions.clone().remove(0);
    }
}

pub fn get_dishes(conn: &mut PgConnection, expansions: Vec<Vec<&str>>) {
    let result: models::Menucard = menucard::dsl::menucard
        .find(id)
        .select(models::Menucard::as_select())
        .first(conn)
        .unwrap();
    let mut api_object: endpoint_models::Menucard = endpoint_models::Menucard {
        dishes: None,
        menucard_id: result.menucard_id,
        name: result.name,
    };

    if expansions.len() > 1 {
        expansions.clone().remove(0);
    }
}

fn match_dish_expansion(
    conn: &mut PgConnection,
    expansion: &str,
    api_object: &mut endpoint_models::Menucard,
) {
    match expansion {
        "menucard" => todo!(),
        &_ => todo!(),
    }
}
