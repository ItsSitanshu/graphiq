use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
  pub id: String,
  pub label: String,
  pub code_hash: String,
  pub input_hashes: HashMap<String, String>,
  pub output_hashes: HashMap<String, String>,
  pub dutation_ms: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExperimentRun {
  pub id: String,
  pub experiment_name: String,
  pub nodes: Vec<Node>,
  pub edges: Vec<Edge>,
  pub created_at: DateTime<Utc>,
  pub total_duration_md: u64,
}