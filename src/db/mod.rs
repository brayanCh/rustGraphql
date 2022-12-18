mod env;

use mongodb::{bson::doc, options::ClientOptions, Client};
use mongodb::{ Database };
use env::{returnMongoKey};


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

//#[tokio::main]
pub async fn initMongoConnection() -> mongodb::error::Result<Database>
{
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse(&returnMongoKey())
            .await?;
    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());

    let client = Client::with_options(client_options)?;

    // Ping the server to see if you can connect to the cluster
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
    println!("Connected successfully.");

    let db : Database = client.database("test");

    // List the names of the databases in that cluster
    Ok(db)
}

