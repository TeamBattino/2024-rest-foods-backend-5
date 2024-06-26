//! This module provides functionality for retrieving settings and their related data from the database.
//!
//! # Functions
//! - `get_setting`: Retrieves the setting, including an optional expansion for the active menucard.
//! - `expand_menucard`: Helper function to expand the active menucard for the setting.

use crate::{endpoint_models, models, schema::setting};
use diesel::{
    query_dsl::methods::SelectDsl, result::Error, PgConnection, RunQueryDsl, SelectableHelper,
};

use super::menucard::get_menucard;

/// Retrieves the setting, including an optional expansion for the active menucard.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// A result containing the setting endpoint model or a Diesel error.
pub fn get_setting(
    conn: &mut PgConnection,
    expansions: &Vec<&str>,
) -> Result<endpoint_models::Setting, Error> {
    let models_setting: models::Setting = setting::dsl::setting
        .select(models::Setting::as_select())
        .first::<models::Setting>(conn)?;

    // expand menucard
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

/// Helper function to expand the active menucard for the setting.
///
/// # Arguments
///
/// * `conn` - A mutable reference to a PostgreSQL connection.
/// * `menucard_id` - The ID of the active menucard to expand.
/// * `expansions` - A vector of strings specifying which related data to expand.
///
/// # Returns
///
/// An option containing the menucard endpoint model.
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
