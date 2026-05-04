use serde::{Serialize, Serializer};

/// Единый тип ошибок приложения.
///
/// Все Tauri-команды возвращают `Result<T, AppError>`.
/// Ошибка автоматически сериализуется в JSON и попадает в JS как Error с понятным сообщением.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Qdrant connection failed: {0}")]
    QdrantConnection(String),

    #[error("Qdrant operation failed: {0}")]
    QdrantOperation(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

/// Tauri требует, чтобы тип ошибки реализовывал `Serialize`,
/// потому что ошибки передаются через IPC в JSON-виде.
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Удобный type alias.
pub type AppResult<T> = Result<T, AppError>;

/// Конвертация ошибок qdrant_client в AppError.
impl From<qdrant_client::QdrantError> for AppError {
    fn from(err: qdrant_client::QdrantError) -> Self {
        AppError::QdrantOperation(err.to_string())
    }
}
