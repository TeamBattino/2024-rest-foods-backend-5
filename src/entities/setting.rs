use crate::{endpoint_models, models, schema::setting};
use diesel::{
    query_dsl::methods::SelectDsl, result::Error, PgConnection, RunQueryDsl, SelectableHelper,
};

use super::menucard::{self, get_menucard};

pub fn get_setting(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Setting, Error> {
    let models_setting: models::Setting = setting::dsl::setting
        .select(models::Setting::as_select())
        .first::<models::Setting>(conn)?;

    // expand dishes
    let menucard = expand_menucard(conn, models_setting.id_menucard_active, expansions);

    let endpoints_setting: endpoint_models::Setting = endpoint_models::Setting {
        menucard_active: menucard,
        id_menucard_active: models_setting.id_menucard_active,
        restaurant_height: models_setting.restaurant_height,
        restaurant_width: models_setting.restaurant_width,
        setting_id: models_setting.setting_id,
    };
    Ok(endpoints_setting)
}
fn expand_menucard(
    conn: &mut PgConnection,
    menucard_id: i32,
    expansions: &Vec<&str>,
) -> Option<endpoint_models::Menucard> {
    if !expansions.contains(&"menucard") {
        return None;
    }

    let menucard_expansions: Vec<&str> = expansions
        .iter()
        .filter(|e| e.starts_with("menucard."))
        .map(|e| &e[9..])
        .collect();

    let endpoints_menucard = get_menucard(conn, menucard_id, &menucard_expansions).unwrap();
    Some(endpoints_menucard)
}
