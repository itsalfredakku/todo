use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Result, Surreal,
};
use std::env;

pub static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

pub async fn connect_db() -> Result<()> {
    // Load the environment variables
    let db_address = env::var("DB_ADDRESS").expect("DB_ADDRESS environment variable is not set");
    let db_user = env::var("DB_USER").expect("DB_USER environment variable is not set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD environment variable is not set");

    // Connect to the database
    let _ = DB.connect::<Ws>(&db_address).await?;
    let _ = DB
        .signin(Root {
            username: &db_user,
            password: &db_password,
        })
        .await?;
    let _ = DB.use_ns("todo").use_db("todo").await?;
    Ok(())
}