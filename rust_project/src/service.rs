use http::StatusCode;

use crate::{
    error::{AppError, AppResult, Reply},
    model::{NewStudent, Student},
    repo::StudentRepo,
    validation,
};

#[derive(Clone)]
pub struct StudentService {
    repo: StudentRepo,
}

impl StudentService {
    pub fn new(repo: StudentRepo) -> Self {
        Self { repo }
    }

    pub async fn create(&self, new_student: NewStudent) -> AppResult<Student> {
        validation::validate_new_student(&new_student)?;

        let student = new_student.into_student();
        self.repo.insert_one(&student).await?;
        Ok(Reply::created(student))
    }

    pub async fn import(&self, students: Vec<Student>) -> AppResult<Reply<usize>> {
        if students.is_empty() {
            return Err(AppError::Validation("the student list is empty".into()));
        }

        let inserted = self.repo.insert_many(students).await?;
        Ok(Reply::created(inserted))
    }

    pub async fn get(&self, id: &str) -> AppResult<Student> {
        let id = validation::parse_id(id)?;

        match self.repo.find_by_id(id).await? {
            Some(student) => Ok(Reply::ok(student)),
            None => Err(AppError::NotFound),
        }
    }

    pub async fn delete(&self, id: &str) -> AppResult<()> {
        let id = validation::parse_id(id)?;

        if self.repo.delete_by_id(id).await? {
            Ok(Reply {
                status: (StatusCode::NO_CONTENT),
                body: (),
            })
        } else {
            Err(AppError::NotFound)
        }
    }

    pub async fn count(&self) -> AppResult<u64> {
        let count = self.repo.count().await?;
        Ok(Reply::ok(count))
    }
}
