//! 工具模块

pub mod file_handler;
pub mod formatter;
pub mod math;

use crate::errors::{AppError, AppResult};

/// 生成唯一ID
pub fn generate_id() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

/// 验证邮箱格式
pub fn validate_email(email: &str) -> AppResult<()> {
    if !email.contains('@') || !email.contains('.') {
        return Err(AppError::ValidationError(
            "邮箱格式不正确".to_string()
        ));
    }
    Ok(())
}

/// 计算百分比
pub fn calculate_percentage(part: f64, total: f64) -> AppResult<f64> {
    if total == 0.0 {
        return Err(AppError::InvalidInput(
            "总数不能为零".to_string()
        ));
    }
    Ok((part / total) * 100.0)
}