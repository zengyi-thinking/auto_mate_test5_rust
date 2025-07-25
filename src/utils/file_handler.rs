//! 文件处理工具

use crate::errors::{AppError, AppResult};
use std::fs;
use std::path::Path;

pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> AppResult<String> {
    fs::read_to_string(path).map_err(AppError::from)
}

pub fn write_string_to_file<P: AsRef<Path>>(path: P, content: &str) -> AppResult<()> {
    fs::write(path, content).map_err(AppError::from)
}

pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> AppResult<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path).map_err(AppError::from)?;
    }
    Ok(())
}

pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_write_and_read_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = "Hello, Rust!";
        
        // 写入文件
        write_string_to_file(&file_path, content).unwrap();
        
        // 读取文件
        let read_content = read_file_to_string(&file_path).unwrap();
        assert_eq!(content, read_content);
    }
    
    #[test]
    fn test_ensure_directory_exists() {
        let dir = tempdir().unwrap();
        let new_dir = dir.path().join("new_directory");
        
        assert!(!new_dir.exists());
        ensure_directory_exists(&new_dir).unwrap();
        assert!(new_dir.exists());
    }
}