use serde::Serialize;
use ts_rs::TS;

use crate::error::AppResult;
use crate::state::AppState;

/// Ответ команды `ping_qdrant`.
///
/// Атрибут `#[derive(TS)]` генерирует TypeScript-определение
/// при выполнении тестов: `cargo test`.
#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../../src/shared/api/bindings/")]
pub struct QdrantPingResponse {
    pub healthy: bool,
    pub version: String,
}

/// Проверяет доступность Qdrant и возвращает его версию.
#[tauri::command]
pub async fn ping_qdrant(
    state: tauri::State<'_, AppState>,
) -> AppResult<QdrantPingResponse> {
    let version = crate::qdrant_client::health_check(&state.qdrant).await?;

    Ok(QdrantPingResponse {
        healthy: true,
        version,
    })
}
