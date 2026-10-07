mod connection;
mod converter;
mod insert;

use insert::sample_student;
use std::error::Error;
use mongodb::bson::doc;
use mongodb::Collection;
use crate::insert::Student;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _convert_bson_to_json = converter::bson_to_json()?;
    let _convert_json_to_vector = converter::json_to_vector()?;
    let _convert_bson_to_vector = converter::bson_to_vector()?;
    let client = connection::conncetion_to_mongodb().await?;
    let status = insert::insert_student(&client, &sample_student()?).await?;
    println!("Status: {status}");
    let coll: Collection<Student> = client.database("School").collection("students_imported");
    println!("Dokumentumok száma: {}", coll.count_documents(doc! {}).await?);

    Ok(())

    // üzenet továbbítása socketre	to_writer vagy to_vec
    // csomagméret mérése / tömörítés	to_vec
    // transzparens módosítás (pl. mező átírása) serde_json::Value / bson::Document
    // debug logolás to_string
    // zlibcompression mongodb tárolási tömörítés
}
