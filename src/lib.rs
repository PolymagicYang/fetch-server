pub mod database;
pub mod models;
pub mod controllers;

use dotenv;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;

pub async fn connect_to_mongo() -> Result<Client, Box<dyn Error>> {
    dotenv::dotenv().ok();
    let mongo_addr = 
        env::var("MONGODB_URL").expect("You must provide the url of mongodb");
    println!("{}", mongo_addr);
    
    let options = 
        ClientOptions::parse_with_resolver_config(&mongo_addr, ResolverConfig::cloudflare())
        .await?;

    let client = Client::with_options(options)?;

    Ok(client)
}