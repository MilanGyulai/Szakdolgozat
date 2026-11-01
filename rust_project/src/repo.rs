use mongodb::{Client, Collection, bson::doc};

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

    pub async fn insert_many(&self, students: Vec<Student>) -> AppResult<usize> {
        if students.is_empty() {
            return Err(AppError::Validation("Empty list".into()));
        }
        let result = self.collection.insert_many(students).await?;
        Ok(Reply::created(result.inserted_ids.len()))
    }

    pub async fn count(&self) -> AppResult<u64> {
        let n = self.collection.count_documents(doc! {}).await?;
        Ok(Reply::ok(n))
    }
}
