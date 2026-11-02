use bson::DateTime;
use mongodb::bson::oid::ObjectId;

use crate::{error::AppError, model::NewStudent};

pub fn parse_id(text: &str) -> Result<ObjectId, AppError> {
    ObjectId::parse_str(text).map_err(|_| AppError::InvalidId(text.to_string()))
}

pub fn validate_new_student(student: &NewStudent) -> Result<(), AppError> {
    let mut problems: Vec<&str> = Vec::new();

    if student.name.trim().is_empty() {
        problems.push("name must not be empty");
    }
    if !is_valid_class(&student.class) {
        problems.push("class must look like 10.B");
    }
    if student.born_in.trim().is_empty() {
        problems.push("born_in must not be empty");
    }
    if student.born_date > DateTime::now() {
        problems.push("born_date must not be in the future");
    }
    if !is_valid_email(&student.email) {
        problems.push("email is not valid");
    }
    if !is_valid_student_number(student.student_number) {
        problems.push("student_number must have 10 digits");
    }
    let phone_is_bad = student
        .phone_number
        .as_deref()
        .is_some_and(|phone| !is_valid_phone(phone));
    if phone_is_bad {
        problems.push("phone_number must look like +36301234567");
    }
    if student.address.city.trim().is_empty() || student.address.street.trim().is_empty() {
        problems.push("address city and street must not be empty");
    }
    if !is_valid_zip_code(&student.address.zip_code) {
        problems.push("zip_code must have 4 digits");
    }
    if !student.math_grades.is_valid() || !student.biology_grades.is_valid() {
        problems.push("grades must be between 1 and 5");
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(AppError::Validation(problems.join("; ")))
    }
}

fn is_valid_class(class: &str) -> bool {
    let Some((grade, letter)) = class.split_once('.') else {
        return false;
    };

    let grade_is_number = !grade.is_empty() && grade.chars().all(|c| c.is_ascii_digit());
    let grade_in_range = matches!(grade.parse::<u8>(), Ok(1..=12));
    let letter_is_valid = letter.len() == 1 && letter.chars().all(|c| c.is_ascii_uppercase());

    grade_is_number && grade_in_range && letter_is_valid
}

fn is_valid_email(email: &str) -> bool {
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };

    !local.is_empty()
        && domain.contains('.')
        && !domain.contains('@')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
        && !email.contains(char::is_whitespace)
}

fn is_valid_student_number(number: i64) -> bool {
    (1_000_000_000..=9_999_999_999).contains(&number)
}

fn is_valid_phone(phone: &str) -> bool {
    let Some(digits) = phone.strip_prefix('+') else {
        return false;
    };
    (9..=15).contains(&digits.len()) && digits.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_zip_code(zip_code: &str) -> bool {
    zip_code.len() == 4 && zip_code.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Address, Grades};

    fn valid_student() -> NewStudent {
        NewStudent {
            name: "Test Student".into(),
            phone_number: Some("+36301234567".into()),
            class: "10.B".into(),
            born_in: "Miskolc".into(),
            born_date: DateTime::parse_rfc3339_str("2010-03-14T00:00:00Z").unwrap(),
            address: Address {
                city: "Miskolc".into(),
                street: "Main street 1.".into(),
                zip_code: "3525".into(),
            },
            student_number: 7_012_345_678,
            email: "test@example.com".into(),
            math_grades: Grades(vec![4, 5]),
            biology_grades: Grades(vec![3]),
        }
    }

    fn problem_text(result: Result<(), AppError>) -> String {
        match result {
            Err(AppError::Validation(text)) => text,
            other => panic!("expected a validation error, got {other:?}"),
        }
    }

    #[test]
    fn valid_student_passes() {
        assert!(validate_new_student(&valid_student()).is_ok());
    }

    #[test]
    fn phone_number_is_optional() {
        let mut student = valid_student();
        student.phone_number = None;
        assert!(validate_new_student(&student).is_ok());
    }

    #[test]
    fn empty_name_is_rejected() {
        let mut student = valid_student();
        student.name = "   ".into();
        assert!(problem_text(validate_new_student(&student)).contains("name"));
    }

    #[test]
    fn grade_out_of_range_is_rejected() {
        let mut student = valid_student();
        student.biology_grades = Grades(vec![0]);
        assert!(problem_text(validate_new_student(&student)).contains("grades"));
    }

    #[test]
    fn future_birth_date_is_rejected() {
        let mut student = valid_student();
        student.born_date = DateTime::parse_rfc3339_str("2999-01-01T00:00:00Z").unwrap();
        assert!(problem_text(validate_new_student(&student)).contains("born_date"));
    }

    #[test]
    fn all_problems_are_reported_together() {
        let mut student = valid_student();
        student.name = String::new();
        student.class = "10B".into();
        let text = problem_text(validate_new_student(&student));
        assert!(text.contains("name"));
        assert!(text.contains("class"));
    }

    #[test]
    fn class_format() {
        assert!(is_valid_class("10.B"));
        assert!(is_valid_class("9.C"));
        assert!(!is_valid_class("10B"));
        assert!(!is_valid_class("10.b"));
        assert!(!is_valid_class("13.A"));
        assert!(!is_valid_class("0.A"));
        assert!(!is_valid_class("+5.A"));
        assert!(!is_valid_class("10.AB"));
        assert!(!is_valid_class(""));
    }

    #[test]
    fn email_format() {
        assert!(is_valid_email("kovacs.anna@pelda.hu"));
        assert!(!is_valid_email("kovacs.anna"));
        assert!(!is_valid_email("@pelda.hu"));
        assert!(!is_valid_email("anna@pelda"));
        assert!(!is_valid_email("anna@@pelda.hu"));
        assert!(!is_valid_email("an na@pelda.hu"));
    }

    #[test]
    fn student_number_needs_ten_digits() {
        assert!(is_valid_student_number(7_012_345_678));
        assert!(!is_valid_student_number(999_999_999));
        assert!(!is_valid_student_number(10_000_000_000));
    }

    #[test]
    fn phone_and_zip_format() {
        assert!(is_valid_phone("+36301234567"));
        assert!(!is_valid_phone("06301234567"));
        assert!(!is_valid_phone("+36abc"));
        assert!(is_valid_zip_code("3525"));
        assert!(!is_valid_zip_code("352"));
        assert!(!is_valid_zip_code("35a5"));
    }

    #[test]
    fn parse_id_accepts_only_object_ids() {
        assert!(parse_id("6acf7bab2684ca57b39e5ddf").is_ok());
        let error = parse_id("not-an-id").unwrap_err();
        assert!(matches!(error, AppError::InvalidId(_)));
    }
}
