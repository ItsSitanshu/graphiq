use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tauri::{AppHandle, Emitter, Runtime};

#[tauri::command]
pub async fn start_python_engine<R: Runtime>(app: AppHandle<R>, script_path: String) -> Result<(), String> {
  let mut child = Command::new("python3")
    .arg("-u")
    .arg(script_path)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| format!("Failed to spawn Python: {}", e))?;

  let stdout = child.stdout.take().expect("Failed to open stdout");
  let stderr = child.stderr.take().expect("Failed to open stderr");

  let app_stdout = app.clone();
  tokio::spawn(async move {
    let mut reader = BufReader::new(stdout).lines();
    while let Ok(Some(line)) = reader.next_line().await {
      if line.starts_with("GRAPHIQ_TRACE:") {
        let json_data = line.trim_start_matches("GRAPHIQ_TRACE:");
        let _ = app_stdout.emit("python-trace", json_data);
      } else {
        let _ = app_stdout.emit("python-log", line);
      }
    }
  });

  let app_stderr = app.clone();
  tokio::spawn(async move {
    let mut reader = BufReader::new(stderr).lines();
    while let Ok(Some(line)) = reader.next_line().await {
      let _ = app_stderr.emit("python-error", line);
    }
  });

  Ok(())
}