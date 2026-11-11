use futures_util::{TryStreamExt};
use mongodb::{
    Client, Collection,
    bson::{Document, doc, oid::ObjectId},
    options::{Collation, ReturnDocument, UpdateModifications},
};

use crate::{config::Config, error::AppError, model::Student, query::StudentFilter};
use crate::model::Subject;

#[derive(Clone)]
pub struct StudentRepo {
    collection: Collection<Student>,
}

impl StudentRepo {

    async fn update_and_return(
        &self,
        id: ObjectId,
        update: impl Into<UpdateModifications>,
    ) -> Result<Option<Student>, AppError> {
        let student = self
            .collection
            .find_one_and_update(doc! { "_id": id }, update)
            .return_document(ReturnDocument::After)
            .await?;
        Ok(student)
    }

    pub async fn push_grade(
        &self,
        id: ObjectId,
        subject: Subject,
        grade: u8,
    ) -> Result<Option<Student>, AppError> {
        self.update_and_return(id, add_grade_pipeline(subject, grade))
            .await
    }

    pub async fn set_fields(
        &self,
        id: ObjectId,
        fields: Document,
    ) -> Result<Option<Student>, AppError> {
        self.update_and_return(id, doc! { "$set": fields }).await
    }

    pub async fn count_matching(&self, filter: &StudentFilter) -> Result<u64, AppError> {
        let count = self.collection.count_documents(filter.to_query()).await?;
        Ok(count)
    }

    pub async fn find_many(&self, filter: &StudentFilter) -> Result<Vec<Student>, AppError> {
        let collection = self
            .collection
            .find(filter.to_query())
            .sort(filter.to_sort())
            .skip(filter.paging.skip())
            .limit(filter.paging.limit())
            .collation(hungarian_collation())
            .await?;

        let students: Vec<Student> = collection.try_collect().await?;
        Ok(students)
    }


    pub async fn new(client: &Client, cfg: &Config) -> Self {
        let database = client.database(&cfg.database);
        Self {
            collection: database.collection(&cfg.collection),
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

    pub async fn insert_many(&self, students: Vec<Student>) -> Result<usize, AppError> {
        let result = self.collection.insert_many(students).await?;
        Ok(result.inserted_ids.len())
    }

    pub async fn count(&self) -> Result<u64, AppError> {
        let count = self.collection.count_documents(doc! {}).await?;
        Ok(count)
    }
}

fn hungarian_collation() -> Collation {
    Collation::builder()
        .locale("hu")
        .numeric_ordering(true)
        .build()
}

fn add_grade_pipeline(subject: Subject, grade: u8) -> Vec<Document> {
    let field = subject.field();

    let mut add_grade = Document::new();
    add_grade.insert(
        field,
        doc! { "$concatArrays": [format!("${field}"), [i32::from(grade)]] },
    );

    let recalculate_gpa = doc! {
            "gpa": {
                "$round": [
                    { "$avg": { "$concatArrays": ["$math_grades", "$biology_grades"] } },
                    2
                ]
            }
        };

    vec![doc! { "$set": add_grade }, doc! { "$set": recalculate_gpa }]
}
