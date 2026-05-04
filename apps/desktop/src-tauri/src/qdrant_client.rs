use qdrant_client::Qdrant;
use crate::error::{AppError, AppResult};

/// URL локального Qdrant в режиме разработки.
const QDRANT_URL: &str = "http://localhost:6333";

/// Создаёт и возвращает клиент Qdrant, готовый к использованию.
///
/// На этапе создания не делается реальное подключение —
/// клиент ленивый, ошибки появятся только при первом запросе.
pub fn create_client() -> AppResult<Qdrant> {
    Qdrant::from_url(QDRANT_URL)
        .build()
        .map_err(|e| AppError::QdrantConnection(e.to_string()))
}

/// Проверяет доступность Qdrant.
///
/// Возвращает строку с версией Qdrant при успехе.
pub async fn health_check(client: &Qdrant) -> AppResult<String> {
    let info = client
        .health_check()
        .await
        .map_err(|e| AppError::QdrantConnection(e.to_string()))?;

    Ok(info.version)
}
