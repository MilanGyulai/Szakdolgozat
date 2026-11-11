use bson::DateTime;
use mongodb::bson::oid::ObjectId;

use crate::{error::AppError, model::NewStudent};
use crate::model::{Address, Grades, StudentPatch};

pub fn parse_id(text: &str) -> Result<ObjectId, AppError> {
    ObjectId::parse_str(text).map_err(|_| AppError::InvalidId(text.to_string()))
}
pub fn validate_grade(grade: u8) -> Result<(), AppError> {
    if (1..=5).contains(&grade) {
        Ok(())
    } else {
        Err(AppError::Validation("grade must be between 1 and 5".into()))
    }
}

pub fn validate_patch(patch: &StudentPatch) -> Result<(), AppError> {
    report([
        patch.name.as_deref().and_then(name_problem),
        patch.class.as_deref().and_then(class_problem),
        patch.born_in.as_deref().and_then(born_in_problem),
        patch.email.as_deref().and_then(email_problem),
        patch.phone_number.as_ref().and_then(|phone| phone.as_deref()).and_then(phone_problem),
        patch.address.as_ref().and_then(address_text_problem),
        patch.address.as_ref().and_then(zip_code_problem),
    ])
}
pub fn validate_new_student(student: &NewStudent) -> Result<(), AppError> {
    report([
        name_problem(&student.name),
        class_problem(&student.class),
        born_in_problem(&student.born_in),
        born_date_problem(&student.born_date),
        email_problem(&student.email),
        student_number_problem(student.student_number),
        student.phone_number.as_deref().and_then(phone_problem),
        address_text_problem(&student.address),
        zip_code_problem(&student.address),
        grades_problem(&student.math_grades, &student.biology_grades),
    ])
}
fn report(problems: impl IntoIterator<Item = Option<&'static str>>) -> Result<(), AppError> {
    let messages: Vec<&str> = problems.into_iter().flatten().collect();

    if messages.is_empty() {
        Ok(())
    } else {
        Err(AppError::Validation(messages.join("; ")))
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

fn name_problem(name: &str) -> Option<&'static str> {
    name.trim().is_empty().then_some("name must not be empty")
}

fn class_problem(class: &str) -> Option<&'static str> {
    (!is_valid_class(class)).then_some("class must look like 10.B")
}

fn born_in_problem(born_in: &str) -> Option<&'static str> {
    born_in.trim().is_empty().then_some("born_in must not be empty")
}

fn born_date_problem(born_date: &DateTime) -> Option<&'static str> {
    (*born_date > DateTime::now()).then_some("born_date must not be in the future")
}

fn email_problem(email: &str) -> Option<&'static str> {
    (!is_valid_email(email)).then_some("email is not valid")
}

fn student_number_problem(number: i64) -> Option<&'static str> {
    (!is_valid_student_number(number)).then_some("student_number must have 10 digits")
}

fn phone_problem(phone: &str) -> Option<&'static str> {
    (!is_valid_phone(phone)).then_some("phone_number must look like +36301234567")
}

fn address_text_problem(address: &Address) -> Option<&'static str> {
    let is_missing = address.city.trim().is_empty() || address.street.trim().is_empty();
    is_missing.then_some("address city and street must not be empty")
}

fn zip_code_problem(address: &Address) -> Option<&'static str> {
    (!is_valid_zip_code(&address.zip_code)).then_some("zip_code must have 4 digits")
}

fn grades_problem(math: &Grades, biology: &Grades) -> Option<&'static str> {
    let is_valid = math.is_valid() && biology.is_valid();
    (!is_valid).then_some("grades must be between 1 and 5")
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
