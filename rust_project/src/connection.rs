use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

pub async fn conncetion_to_mongodb() -> mongodb::error::Result<Client> {
    let url = "mongodb://localhost:27017/";
    let mut client_options = ClientOptions::parse(url).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;
    println!("Pinged your the server. You successfully connected to MongoDB!");

    Ok(client)
}
