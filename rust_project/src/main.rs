mod config;
mod connection;
mod error;
mod model;
mod repo;
mod seed;

use crate::{config::Config, repo::StudentRepo};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env();
    println!("Connecting {}", config.uri);

    let client = connection::connect(&config).await?;

    let repo = StudentRepo::new(&client, &config);

    if std::env::args().nth(1).as_deref() == Some("seed") {
        let reply = repo.insert_many(seed::sample_students()?).await?;
        println!("{} -> {} student inserted", reply.status, reply.body);
    }

    let reply = repo.count().await?;
    println!(
        "{} -> {} Student already in the database",
        reply.status, reply.body
    );

    Ok(())

    // TODO:
    // NOTE:
    //  üzenet továbbítása socketre	to_writer vagy to_vec

    // csomagméret mérése / tömörítés	to_vec
    // transzparens módosítás (pl. mező átírása) serde_json::Value / bson::Document
    // debug logolás to_string
    // zlibcompression mongodb tárolási tömörítés
    // külön mappába mehet majd /Programs/
}
