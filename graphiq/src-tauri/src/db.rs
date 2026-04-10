use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use serde_json;
use crate::models::ExperimentRun;

pub struct DbManager {
  pub pool: Pool<Sqlite>,
}

impl DbManager {
  pub async fn new(app_dir: &std::path::Path) -> Self {
    let abs_path = std::fs::canonicalize(app_dir)
      .unwrap_or_else(|_| app_dir.to_path_buf());
  
    let db_path = abs_path.join("graphiq.db");
    
    let db_url = format!("sqlite:{}?mode=rwc", db_path.to_str().expect("Path UTF8 error"));
    
    println!("Attempting to open DB at: {}", db_url);

    let pool = SqlitePool::connect(&db_url)
      .await
      .expect("Failed to connect to SQLite. Check folder permissions.");

    sqlx::query(
      "CREATE TABLE IF NOT EXISTS runs (
        id TEXT PRIMARY KEY,
        experiment_name TEXT NOT NULL,
        data JSON NOT NULL,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
      )"
    )
    .execute(&pool)
    .await
    .expect("Failed to create tables");

    Self { pool }
  }

  pub async fn save_run(&self, run: ExperimentRun) -> Result<(), sqlx::Error> {
    let serialized = serde_json::to_string(&run).unwrap();
    
    sqlx::query("INSERT INTO runs (id, experiment_name, data) VALUES (?, ?, ?)")
      .bind(&run.id)
      .bind(&run.experiment_name)
      .bind(serialized)
      .execute(&self.pool)
      .await?;
        
    Ok(())
  }

  pub async fn get_all_runs(&self, experiment_name: &str) -> Vec<ExperimentRun> {
    let rows: Vec<(String,)> = sqlx::query_as("SELECT data FROM runs WHERE experiment_name = ?")
      .bind(experiment_name)
      .fetch_all(&self.pool)
      .await
      .unwrap_or_default();

    rows.into_iter()
      .map(|(json_str,)| serde_json::from_str(&json_str).unwrap())
      .collect()
  }
}