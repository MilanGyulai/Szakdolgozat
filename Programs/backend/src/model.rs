use std::str::FromStr;
use bson::{DateTime, oid::ObjectId, Document};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use error::AppError;
use crate::error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub city: String,
    pub street: String,
    pub zip_code: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Grades(pub Vec<u8>);

impl Grades {
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|grade| (1..=5).contains(grade))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subject {
    Math,
    Biology,
}

impl Subject {
    /// The field name in the database.
    pub fn field(self) -> &'static str {
        match self {
            Subject::Math => "math_grades",
            Subject::Biology => "biology_grades",
        }
    }
}

impl FromStr for Subject {
    type Err = AppError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text.to_lowercase().as_str() {
            "math" => Ok(Subject::Math),
            "biology" => Ok(Subject::Biology),
            other => Err(AppError::Validation(format!("unknown subject: {other}"))),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct StudentPatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub born_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl StudentPatch {
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    pub fn to_set_document(&self) -> Result<Document, AppError> {
        Ok(bson::serialize_to_document(self)?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Student {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub phone_number: Option<String>,
    pub class: String,
    pub born_in: String,
    pub born_date: DateTime,
    pub is_active: bool,
    #[serde(rename = "is_enrolled")]
    pub enrolled_at: DateTime,
    pub address: Address,
    pub student_number: i64,
    pub email: String,
    pub math_grades: Grades,
    pub biology_grades: Grades,
    pub gpa: f64,
}

impl Student {
    pub fn compute_gpa(&self) -> f64 {
        let all: Vec<u32> = self
            .math_grades
            .0
            .iter()
            .chain(&self.biology_grades.0)
            .map(|&grades| u32::from(grades))
            .collect();
        if all.is_empty() {
            return 0.0;
        }
        let avg = f64::from(all.iter().sum::<u32>()) / all.len() as f64;
        (avg * 100.0).round() / 100.0
    }

    pub fn recalc_gpa(&mut self) {
        self.gpa = self.compute_gpa();
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn age_on(&self, today: NaiveDate) -> Option<u32> {
        let millis = self.born_date.timestamp_millis();
        let born = chrono::DateTime::from_timestamp_millis(millis)?.date_naive();
        today.years_since(born)
    }

    #[expect(dead_code, reason = "TODO")]
    pub fn age(&self) -> Option<u32> {
        self.age_on(Utc::now().date_naive())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewStudent {
    pub name: String,
    pub phone_number: Option<String>,
    pub class: String,
    pub born_in: String,
    pub born_date: DateTime,
    pub address: Address,
    pub student_number: i64,
    pub email: String,
    #[serde(default)]
    pub math_grades: Grades,
    #[serde(default)]
    pub biology_grades: Grades,
}

impl NewStudent {
    pub fn into_student(self) -> Student {
        let mut student = Student {
            id: ObjectId::new(),
            name: self.name,
            phone_number: self.phone_number,
            class: self.class,
            born_in: self.born_in,
            born_date: self.born_date,
            is_active: true,
            enrolled_at: DateTime::now(),
            address: self.address,
            student_number: self.student_number,
            email: self.email,
            math_grades: self.math_grades,
            biology_grades: self.biology_grades,
            gpa: 0.0,
        };
        student.recalc_gpa();
        student
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Student {
        Student {
            id: ObjectId::new(),
            name: "Teszt Elek".into(),
            phone_number: None,
            class: "10.B".into(),
            born_in: "Miskolc".into(),
            born_date: DateTime::parse_rfc3339_str("2010-03-14T00:00:00Z").unwrap(),
            is_active: true,
            enrolled_at: DateTime::parse_rfc3339_str("2024-09-02T08:00:00Z").unwrap(),
            address: Address {
                city: "Miskolc".into(),
                street: "Fő utca 1.".into(),
                zip_code: "3525".into(),
            },
            student_number: 7_012_345_678,
            email: "teszt@pelda.hu".into(),
            math_grades: Grades(vec![4, 3, 4, 5]),
            biology_grades: Grades(vec![3, 4, 4]),
            gpa: 0.0,
        }
    }

    #[test]
    fn gpa_is_average_of_all_grades() {
        assert_eq!(sample().compute_gpa(), 3.86);
    }

    #[test]
    fn gpa_of_empty_is_zero() {
        let mut s = sample();
        s.math_grades = Grades(vec![]);
        s.biology_grades = Grades(vec![]);
        assert_eq!(s.compute_gpa(), 0.0);
    }

    #[test]
    fn age_before_and_after_birthday() {
        let s = sample();
        assert_eq!(
            s.age_on(NaiveDate::from_ymd_opt(2026, 3, 13).unwrap()),
            Some(15)
        );
        assert_eq!(
            s.age_on(NaiveDate::from_ymd_opt(2026, 3, 14).unwrap()),
            Some(16)
        );
    }

    #[test]
    fn roundtrip_keeps_field_names() {
        let doc = bson::serialize_to_document(&sample()).unwrap();
        assert!(doc.contains_key("_id"));
        assert!(doc.contains_key("is_enrolled"));
        assert!(!doc.contains_key("age"));
        let back: Student = bson::deserialize_from_document(doc).unwrap();
        assert_eq!(back, sample_with_id(back.id));
    }

    fn sample_with_id(id: ObjectId) -> Student {
        Student { id, ..sample() }
    }
}
