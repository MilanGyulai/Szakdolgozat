use bson::{DateTime, oid::ObjectId};

use crate::{
    error::AppError,
    model::{Address, Grades, Student},
};

pub fn sample_students() -> Result<Vec<Student>, AppError> {
    let mut students = vec![
        Student {
            id: ObjectId::new(),
            name: "Kovács Anna".into(),
            phone_number: None,
            class: "10.B".into(),
            born_in: "Miskolc".into(),
            born_date: DateTime::parse_rfc3339_str("2010-03-14T00:00:00Z")?,
            is_active: true,
            enrolled_at: DateTime::parse_rfc3339_str("2024-09-02T08:00:00Z")?,
            address: Address {
                city: "Miskolc".into(),
                street: "Széchenyi István út 12.".into(),
                zip_code: "3525".into(),
            },
            student_number: 7_012_345_678,
            email: "kovacs.anna@pelda.hu".into(),
            math_grades: Grades(vec![4, 3, 4, 5]),
            biology_grades: Grades(vec![3, 4, 4]),
            gpa: 0.0,
        },
        Student {
            id: ObjectId::new(),
            name: "Nagy Péter".into(),
            phone_number: Some("+36301234567".into()),
            class: "11.A".into(),
            born_in: "Budapest".into(),
            born_date: DateTime::parse_rfc3339_str("2009-07-21T00:00:00Z")?,
            is_active: true,
            enrolled_at: DateTime::parse_rfc3339_str("2023-09-01T08:00:00Z")?,
            address: Address {
                city: "Miskolc".into(),
                street: "Egyetemváros A/2".into(),
                zip_code: "3515".into(),
            },
            student_number: 7_098_765_432,
            email: "nagy.peter@pelda.hu".into(),
            math_grades: Grades(vec![5, 5, 4, 5]),
            biology_grades: Grades(vec![4, 5, 5]),
            gpa: 0.0,
        },
        Student {
            id: ObjectId::new(),
            name: "Szabó Eszter".into(),
            phone_number: None,
            class: "9.C".into(),
            born_in: "Debrecen".into(),
            born_date: DateTime::parse_rfc3339_str("2011-11-05T00:00:00Z")?,
            is_active: false,
            enrolled_at: DateTime::parse_rfc3339_str("2025-09-01T08:00:00Z")?,
            address: Address {
                city: "Miskolc".into(),
                street: "Kossuth Lajos utca 8.".into(),
                zip_code: "3530".into(),
            },
            student_number: 7_055_443_322,
            email: "szabo.eszter@pelda.hu".into(),
            math_grades: Grades(vec![3, 4, 3]),
            biology_grades: Grades(vec![5, 4, 4, 5]),
            gpa: 0.0,
        },
    ];

    for s in &mut students {
        s.recalc_gpa();
    }

    Ok(students)
}
