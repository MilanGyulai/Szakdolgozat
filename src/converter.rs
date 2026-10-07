use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use bson::{Bson, Document};
use serde_json::Value;

pub fn bson_to_json() -> Result<(), Box<dyn Error>> {
    let file = File::open("./TestFiles/tanulo.bson")?;
    let doc = Document::from_reader(file)?;

    let value = Bson::Document(doc).into_relaxed_extjson();

    let out = BufWriter::new(File::create("./TestFiles/tanulo.json")?);
    serde_json::to_writer_pretty(out, &value)?;
    Ok(())
}
pub fn bson_to_vector() -> Result<Vec<u8>, Box<dyn Error>> {
    let doc = Document::from_reader(File::open("./TestFiles/tanulo.bson")?)?;

    let mut bytes: Vec<u8> = Vec::new();
    doc.to_writer(&mut bytes)?;

    let out = BufWriter::new(File::create("./TestFiles/tanulo_byte_bson.txt")?);
    serde_json::to_writer_pretty(out, &bytes)?;

    Ok(bytes)
}

pub fn json_to_vector() -> Result<Vec<u8>, Box<dyn Error>> {
    let file = File::open("./TestFiles/tanulo.json")?;
    let doc: Value = serde_json::from_reader(file)?;

    let bytes = serde_json::to_vec(&doc)?;

    let out = BufWriter::new(File::create("./TestFiles/tanulo_byte_json.txt")?);
    serde_json::to_writer_pretty(out, &bytes)?;

    Ok(bytes)
}