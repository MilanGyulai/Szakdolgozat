
    use std::error::Error;
    use bson::DateTime;
    use bson::oid::ObjectId;
    use http::StatusCode;
    use mongodb::{Client, Collection, bson::doc};
    use serde::{Deserialize, Serialize};

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
    _id: ObjectId,
    name: String,
    phone_number: Option<String>,
    class: String,
    born_in: String,
    born_date: DateTime,
    is_active: bool,
    is_enrolled: DateTime,
    address: Address,
    student_number: u64,
    email: String,
    math_grades: Grades,
    biology_grades: Grades,
    age: u8,
    gpa: f64,
}

pub async fn insert_student(client: &Client,student: &Student ) -> Result<http::StatusCode, mongodb::error::Error> {
    let collection: Collection<Student> = client.database("School").collection("students_imported");
    collection.insert_one(student).await?;
    Ok(StatusCode::CREATED)
}

pub fn sample_student() -> Result<Student, Box<dyn Error>> {
    Ok(Student {
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
    })
}
