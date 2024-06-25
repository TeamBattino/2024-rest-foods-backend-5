use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::schema::setting;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("Connection to the database established!");

    return connection;
}

/* pub fn get_db_settings(conn: &mut PgConnection) -> Setting {
    setting::dsl::setting
        .select(Setting::as_select())
        .first(conn)
        .expect("Error loading settings")
}
 */