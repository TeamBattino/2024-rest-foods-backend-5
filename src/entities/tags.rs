use crate::schema::tag;
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
    // Todo expansion for Dishes
    let endpoints_tag: endpoint_models::Tag = endpoint_models::Tag {
        name: models_tag.name,
        dishes: None,
        tag_id: models_tag.tag_id,
    };
    Ok(endpoints_tag)
}
