use crate::schema::menucard::{self, menucard_id};
use crate::{endpoint_models, models};
use diesel::dsl::select;
use diesel::{prelude::*, result};
use diesel::PgConnection;

enum IdType {
    Single(i32),
    Multiple(Vec<i32>),
    All,
}

pub fn get_menucard(conn: &mut PgConnection, expansions: Vec<Vec<&str>>, id: IdType) -> Vec<models::Menucard> {
    let result = match id {
        IdType::Single(single_id) => {
            let menucard = menucard::dsl::menucard
                .find(single_id)
                .select(models::Menucard::as_select())
                .first::<models::Menucard>(conn)
                .expect("Error loading menucard");
            vec![menucard]
        }
        IdType::Multiple(ids) => {
            let menucards = menucard::dsl::menucard
                .filter(menucard::dsl::id.eq_any(ids))
                .select(models::Menucard::as_select())
                .load::<models::Menucard>(conn)
                .expect("Error loading menucards");
            menucards
        }
        IdType::All => {
            // Implement logic to return all menucards
            let menucards = menucard::dsl::menucard
                .select(models::Menucard::as_select())
                .load::<models::Menucard>(conn)
                .expect("Error loading all menucards");
            menucards
        }
    };
    for current in result {
        
    }
    return result;
}

    let mut api_object: endpoint_models::Menucard = endpoint_models::Menucard {
        dishes: None,
        menucard_id: result.menucard_id,
        name: result.name,
    };

    if expansions.len() > 1 {
        expansions.clone().remove(0);
    }
}

pub fn get_menucards(conn: &mut PgConnection, id: i32, expansions: Vec<Vec<&str>>) {
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
