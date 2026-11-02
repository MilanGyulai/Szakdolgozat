use mongodb::{
    Client, Collection,
    bson::{doc, oid::ObjectId},
};

use crate::{
    config::Config,
    error::{AppError, AppResult, Reply},
    model::Student,
};

#[derive(Clone)]
pub struct StudentRepo {
    collection: Collection<Student>,
}

impl StudentRepo {
    pub fn new(client: &Client, cfg: &Config) -> Self {
        Self {
            collection: client.database(&cfg.database).collection(&cfg.collection),
        }
    }

    pub async fn delete_by_id(&self, id: ObjectId) -> Result<bool, AppError> {
        let result = self.collection.delete_one(doc! {"_id": id}).await?;
        Ok(result.deleted_count == 1)
    }

    pub async fn find_by_id(&self, id: ObjectId) -> Result<Option<Student>, AppError> {
        let student = self.collection.find_one(doc! {"_id": id}).await?;
        Ok(student)
    }

    pub async fn insert_one(&self, students: &Student) -> Result<(), AppError> {
        self.collection.insert_one(students).await?;
        Ok(())
    }

    pub async fn insert_many(&self, students: Vec<Student>) -> AppResult<usize> {
        if students.is_empty() {
            return Err(AppError::Validation("Empty list".into()));
        }
        let result = self.collection.insert_many(students).await?;
        Ok(Reply::created(result.inserted_ids.len()))
    }

    pub async fn count(&self) -> Result<u64, AppError> {
        let count = self.collection.count_documents(doc! {}).await?;
        Ok(count)
    }
}
