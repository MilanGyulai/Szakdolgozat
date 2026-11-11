//! NOT USED IN THIS VERSION
//! Replaced by model.rs, repo.rs and student_seed.
use bson::DateTime;
use bson::oid::ObjectId;
use http::StatusCode;
use mongodb::{Client, Collection, bson::doc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub city: String,
    pub street: String,
    pub zip_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grades(pub Vec<u8>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Student {
    pub _id: ObjectId,
    pub name: String,
    pub phone_number: Option<String>,
    pub class: String,
    pub born_in: String,
    pub born_date: DateTime,
    pub is_active: bool,
    pub is_enrolled: DateTime,
    pub address: Address,
    pub student_number: u64,
    pub email: String,
    pub math_grades: Grades,
    pub biology_grades: Grades,
    pub age: u8,
    pub gpa: f64,
}

pub async fn insert_student(
    client: &Client,
    student: Vec<Student>,
) -> Result<http::StatusCode, mongodb::error::Error> {
    let collection: Collection<Student> = client.database("School").collection("students_imported");
    collection.insert_many(student).await?;
    Ok(StatusCode::CREATED)
}

pub fn sample_student() -> Result<Vec<Student>, Box<dyn Error>> {
    Ok(vec![
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
    ])
}
