mod commands;
mod error;
mod qdrant_client;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Инициализация логирования.
    // Уровень контролируется переменной окружения RUST_LOG.
    // Например: RUST_LOG=desktop=debug,info
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    tracing::info!("Starting Buh Buddy desktop application");

    // Создаём клиент Qdrant.
    // Если Qdrant недоступен в момент старта — приложение всё равно запустится,
    // ошибка будет видна только при первом запросе (через ping_qdrant).
    let qdrant = qdrant_client::create_client()
        .expect("Failed to create Qdrant client");

    let app_state = AppState::new(qdrant);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![commands::system::ping_qdrant])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
