mod models;
mod db;
mod hashing;

use db::DbManager;
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
  db: Arc<Mutex<DbManager>>,
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_artifact_hash(path: String) -> Result<String, String> {
  hashing::hash_artifact(&path)
}


#[tauri::command]
async fn save_experiment_result(
  state: State<'_, AppState>,
  run: models::ExperimentRun
) -> Result<(), String> {
  let db = state.db.lock().await;
  db.save_run(run).await.map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let home_dir = std::env::var("HOME").expect("Could not find home directory");
  let base_path = std::path::PathBuf::from(home_dir).join(".graphiq_dev_data");

  if !base_path.exists() {
    std::fs::create_dir_all(&base_path).expect("CRITICAL: Failed to create .graphiq_dev_data in home");
  }

  println!("Success: Data Vault located at {:?}", base_path);

  let db_manager = tauri::async_runtime::block_on(async {
    db::DbManager::new(&base_path).await
  });
  
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .manage(AppState {
      db: Arc::new(Mutex::new(db_manager)),
    })
    .invoke_handler(tauri::generate_handler![
      save_experiment_result,
      get_artifact_hash
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}