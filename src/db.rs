use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::schema::setting;
use crate::Setting;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    println!("Connection to the database established!");

    return connection;
}

pub fn get_settings()  {
   let connection = &mut establish_connection();
    let results = setting::dsl::setting
        .select(Setting::as_select())
        .limit(5)
        .load(connection)
        .expect("Error loading menucards");
    for result in &results {
        println!("{}", result.setting_id);
        println!("{}", result.id_menucard_active);
        println!("{}", result.restaurant_width);
        println!("{}", result.restaurant_height);
    }
}
