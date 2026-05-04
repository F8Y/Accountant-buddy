use qdrant_client::Qdrant;

/// Состояние приложения, доступное всем Tauri-командам.
///
/// Создаётся один раз при запуске и хранит долгоживущие ресурсы:
/// клиент Qdrant, ML-модели (когда добавим), HTTP-клиент и т.д.
pub struct AppState {
    pub qdrant: Qdrant,
}

impl AppState {
    pub fn new(qdrant: Qdrant) -> Self {
        Self { qdrant }
    }
}
