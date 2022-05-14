use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
        SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Just testing that we can connect to the DB.
// Database starts empty, you may need to manually connect to populate some initial rows.
fn main() {
    let connection = establish_connection();
    println!("Connection established");
    use custos::models::Channel;
    use custos::schema::channels::dsl::*;
    let results = channels.load::<Channel>(&connection).expect("error loading channels");
    for channel in results {
        dbg!(channel);
    }
}