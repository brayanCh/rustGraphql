use mongodb::{bson::doc, options::ClientOptions, Client};
mod env;
use env::{returnMongoKey};


//#[tokio::main]
pub async fn initMongoConnection() -> mongodb::error::Result<()>
{
    println!("initMongoStarted");
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

    let db = client.database("test");
    // List the names of the databases in that cluster
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    Ok(())
}
