use dotenv::dotenv;
use mysql::*;
use std::env;

pub fn establish_connection() -> Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = Pool::new(database_url.as_str()).unwrap();

    return pool;
}
