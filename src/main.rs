use db::establish_connection;

mod db;
mod schema;
use self::models::*;

fn main() {
    establish_connection()
}
