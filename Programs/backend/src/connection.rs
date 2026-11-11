use std::time::Duration;

use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

use crate::{config::Config, error::AppError};

pub async fn connect(config: &Config) -> Result<Client, AppError> {
    let mut options = ClientOptions::parse(&config.uri).await?;

    options.app_name = Some(config.app_name.clone());
    options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());
    options.server_selection_timeout = Some(Duration::from_secs(3));

    let client = Client::with_options(options)?;

    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    Ok(client)
}
