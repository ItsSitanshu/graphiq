use blake3::Hasher;
use std::fs::{self, File};
use std::path::Path;
use std::io::{Read, Seek, SeekFrom};

pub fn hash_artifact(path: &str) -> Result<String, String> {
  let path_buf = Path::new(path);
  if !path_buf.exists() {
    return Err(format!("File not found: {}", path));
  }

  let metadata = fs::metadata(path_buf).map_err(|e| e.to_string())?;
  let file_size = metadata.len();

  if file_size < 100 * 1024 * 1024 {
    full_hash(path_buf)
  } else {
    fingerprint_hash(path_buf, file_size)
  }
}

fn full_hash(path: &Path) -> Result<String, String> {
  let mut hasher = Hasher::new();
  hasher.update_mmap(path).map_err(|e| format!("Mmap Error: {}", e))?;
  Ok(hasher.finalize().to_hex().to_string())
}

fn fingerprint_hash(path: &Path, size: u64) -> Result<String, String> {
  let mut file = File::open(path).map_err(|e| e.to_string())?;
  let mut hasher = Hasher::new();
  let mut buffer = [0u8; 65536]; 

  hasher.update(size.to_string().as_bytes());

  file.read_exact(&mut buffer).ok();
  hasher.update(&buffer);

  if size > 128 * 1024 {
    file.seek(SeekFrom::Start(size / 2)).map_err(|e| e.to_string())?;
    file.read_exact(&mut buffer).ok();
    hasher.update(&buffer);
  }

  if size > 65536 {
    file.seek(SeekFrom::End(-65536)).map_err(|e| e.to_string())?;
    file.read_exact(&mut buffer).ok();
    hasher.update(&buffer);
  }

  Ok(hasher.finalize().to_hex().to_string())
}