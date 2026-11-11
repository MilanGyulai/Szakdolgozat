use http::StatusCode;
use mongodb::error::{ErrorKind, WriteFailure};

const DUPLICATE_KEY: i32 = 11000;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad Id: {0}")]
    InvalidId(String),

    #[error("Bad data: {0}")]
    Validation(String),

    #[error("Not found")]
    NotFound,

    #[error("Exists: {0}")]
    Duplicate(String),

    #[error("Unavailable")]
    Unavailable,

    #[error("Database error: {0}")]
    Db(mongodb::error::Error),

    #[error("BSON hiba: {0}")]
    Bson(#[from] bson::error::Error),
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::InvalidId(_) => StatusCode::BAD_REQUEST,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Duplicate(_) => StatusCode::CONFLICT,
            AppError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Db(_) | AppError::Bson(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(e: mongodb::error::Error) -> Self {
        classify(&e).unwrap_or(AppError::Db(e))
    }
}

fn classify(e: &mongodb::error::Error) -> Option<AppError> {
    match &*e.kind {
        ErrorKind::Write(WriteFailure::WriteError(w)) if w.code == DUPLICATE_KEY => {
            Some(AppError::Duplicate(w.message.clone()))
        }

        ErrorKind::InsertMany(im) => im
            .write_errors
            .iter()
            .flatten()
            .find(|w| w.code == DUPLICATE_KEY)
            .map(|w| AppError::Duplicate(w.message.clone())),
        ErrorKind::Command(c) if c.code == DUPLICATE_KEY => {
            Some(AppError::Duplicate(c.message.clone()))
        }
        ErrorKind::ServerSelection { .. }
        | ErrorKind::Io(_)
        | ErrorKind::ConnectionPoolCleared { .. } => Some(AppError::Unavailable),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Reply<T> {
    pub status: StatusCode,
    pub body: T,
}

impl<T> Reply<T> {
    pub fn ok(body: T) -> Self {
        Self {
            status: StatusCode::OK,
            body,
        }
    }

    pub fn created(body: T) -> Self {
        Self {
            status: StatusCode::CREATED,
            body,
        }
    }
}

pub type AppResult<T> = Result<Reply<T>, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_mapping() {
        assert_eq!(AppError::NotFound.status(), StatusCode::NOT_FOUND);
        assert_eq!(
            AppError::Duplicate("x".into()).status(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            AppError::Validation("x".into()).status(),
            StatusCode::UNPROCESSABLE_ENTITY
        );
        assert_eq!(
            AppError::InvalidId("x".into()).status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            AppError::Unavailable.status(),
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    #[test]
    fn reply_codes() {
        assert_eq!(Reply::ok(1).status, StatusCode::OK);
        assert_eq!(Reply::created(1).status, StatusCode::CREATED);
    }
}
