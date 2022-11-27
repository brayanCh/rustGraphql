use mongodb::{bson::doc, options::ClientOptions, Client};
mod env;
use tokio;
use env::{returnMongoKey};


#[tokio::main]
pub async fn initMongoConnection() -> mongodb::error::Result<()>
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
    // List the names of the databases in that cluster
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
