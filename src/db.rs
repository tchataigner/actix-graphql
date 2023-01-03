use mongodb::Client;
use mongodb::options::{ClientOptions, ResolverConfig};

pub async fn get_db_client() -> Client {
    let db_url = std::env::var("MONGODB_CONNSTRING").expect("MONGODB_CONNSTRING must be set");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&db_url, ResolverConfig::cloudflare())
            .await.unwrap();
    Client::with_options(options).unwrap()
}