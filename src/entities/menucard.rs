use crate::schema::menucard::{self, menucard_id};
use crate::{endpoint_models, models};
use diesel::result::Error;
use diesel::PgConnection;
use diesel::{prelude::*, result};

enum IdType {
    Single(i32),
    Multiple(Vec<i32>),
    All,
}

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

fn match_menucard_expansion(
    conn: &mut PgConnection,
    expansion: &str,
    api_object: &mut endpoint_models::Menucard,
) {
    match expansion {
        "menucard" => todo!(),
        &_ => todo!(),
    }
}
