use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use bson::{Bson, Document, oid::ObjectId, DateTime};
use serde_json::Value;

use crate::insert::Student;
use crate::insert::Grades;
use crate::insert::Address;

//TODO:Megcsinálni minden adat exportálására Vector makróként átadva
//TODO: Esetleg a func-nak a bevitele lehet maga a fájlnév.
pub fn bson_to_relaxed_json() -> Result<(), Box<dyn Error>> {
    let file = File::open("./megfigyelesek/data_to_bson.bson")?;
    let doc = Document::from_reader(file)?;

    let value = Bson::Document(doc).into_relaxed_extjson();

    let out = BufWriter::new(File::create("./megfigyelesek/tanulo_relaxed.json")?);
    serde_json::to_writer_pretty(out, &value)?;
    Ok(())
}

pub fn bson_to_canonical_json() -> Result<(), Box<dyn Error>> {
    let file = File::open("./TestFiles/tanulo.bson")?;
    let doc = Document::from_reader(file)?;

    let value = Bson::Document(doc).into_canonical_extjson();

    let out = BufWriter::new(File::create("./TestFiles/tanulo_canonical.json")?);
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

pub fn data_to_bson() -> Result<Vec<Student>, Box<dyn Error>>{

    let students =vec![
        Student {
        _id: ObjectId::new(),
        name: "Kovács Anna".into(),
        phone_number: None,
        class: "10.B".into(),
        born_in: "Miskolc".into(),
        born_date: DateTime::parse_rfc3339_str("2010-03-14T00:00:00Z")?,
        is_active: true,
        is_enrolled: DateTime::parse_rfc3339_str("2024-09-02T08:00:00Z")?,
        address: Address {
            city: "Miskolc".into(),
            street: "Széchenyi István út 12.".into(),
            zip_code: "3525".into(),
        },
        student_number: 7_012_345_678,
        email: "kovacs.anna@pelda.hu".into(),
        math_grades: Grades(vec![4, 3, 4, 5]),
        biology_grades: Grades(vec![3, 4, 4]),
        age: 16,
        gpa: 3.8,
    },
        Student {
            _id: ObjectId::new(),
                name: "Nagy Péter".into(),
                phone_number: Some("+36301234567".into()),
                class: "11.A".into(),
                born_in: "Budapest".into(),
                born_date: DateTime::parse_rfc3339_str("2009-07-21T00:00:00Z")?,
                is_active: true,
                is_enrolled: DateTime::parse_rfc3339_str("2023-09-01T08:00:00Z")?,
                address: Address {
                    city: "Miskolc".into(),
                    street: "Egyetemváros A/2".into(),
                    zip_code: "3515".into(),
                },
                student_number: 7_098_765_432,
                email: "nagy.peter@pelda.hu".into(),
                math_grades: Grades(vec![5, 5, 4, 5]),
                biology_grades: Grades(vec![4, 5, 5]),
                age: 17,
                gpa: 4.6,
            },
            Student {
                _id: ObjectId::new(),
                name: "Szabó Eszter".into(),
                phone_number: None,
                class: "9.C".into(),
                born_in: "Debrecen".into(),
                born_date: DateTime::parse_rfc3339_str("2011-11-05T00:00:00Z")?,
                is_active: false,
                is_enrolled: DateTime::parse_rfc3339_str("2025-09-01T08:00:00Z")?,
                address: Address {
                    city: "Miskolc".into(),
                    street: "Kossuth Lajos utca 8.".into(),
                    zip_code: "3530".into(),
                },
                student_number: 7_055_443_322,
                email: "szabo.eszter@pelda.hu".into(),
                math_grades: Grades(vec![3, 4, 3]),
                biology_grades: Grades(vec![5, 4, 4, 5]),
                age: 15,
                gpa: 4.1,
            },
    ];

    let mut out = BufWriter::new(File::create("./TestFiles/data_to_bson.bson")?);

    for students in &students{
        let doc =bson::serialize_to_document(students)?;
        doc.to_writer(&mut out)?;
    }
    println!("Bson from data was created!");

    Ok(students)
}
