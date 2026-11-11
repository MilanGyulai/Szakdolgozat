use http::StatusCode;

use crate::{
    error::{AppError, AppResult, Reply},
    model::{NewStudent, Student},
    repo::StudentRepo,
    validation,
};
use crate::model::{StudentPatch, Subject};
use crate::query::{Page, StudentFilter};

#[derive(Clone)]
pub struct StudentService {
    repo: StudentRepo,
}

impl StudentService {
    pub fn new(repo: StudentRepo) -> Self {
        Self { repo }
    }

    pub async fn add_grade(&self, id: &str, subject: Subject, grade: u8) -> AppResult<Student> {
        let id = validation::parse_id(id)?;
        validation::validate_grade(grade)?;

        found(self.repo.push_grade(id, subject, grade).await?)
    }
    pub async fn set_active(&self, id: &str, is_active: bool) -> AppResult<Student> {
        let patch = StudentPatch { is_active: Some(is_active), ..Default::default() };
        self.update(id, patch).await
    }

    pub async fn update(&self, id: &str, patch: StudentPatch) -> AppResult<Student> {
        let id = validation::parse_id(id)?;

        if patch.is_empty() {
            return Err(AppError::Validation("nothing to update".into()));
        }
        validation::validate_patch(&patch)?;

        let fields = patch.to_set_document()?;
        found(self.repo.set_fields(id, fields).await?)
    }

    pub async fn list(&self, filter: &StudentFilter) -> AppResult<Page<Student>> {
        let (items, total) = tokio::try_join!(
            self.repo.find_many(filter),
            self.repo.count_matching(filter)
        )?;

        let page = Page {
            items,
            page: filter.paging.page(),
            per_page: filter.paging.per_page(),
            total,
        };
        Ok(Reply::ok(page))
    }
    pub async fn create(&self, new_student: NewStudent) -> AppResult<Student> {
        validation::validate_new_student(&new_student)?;

        let student = new_student.into_student();
        self.repo.insert_one(&student).await?;
        Ok(Reply::created(student))
    }

    pub async fn import(&self, students: Vec<Student>) -> AppResult<usize> {
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

fn found(student: Option<Student>) -> AppResult<Student> {
    student.map(Reply::ok).ok_or(AppError::NotFound)
}
