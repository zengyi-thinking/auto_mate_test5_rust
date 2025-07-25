//! 错误处理模块
//! 
//! 定义项目中使用的所有错误类型

use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    NotFound(String),
    ValidationError(String),
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "输入错误: {}", msg),
            AppError::NotFound(msg) => write!(f, "未找到: {}", msg),
            AppError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            AppError::IoError(err) => write!(f, "IO错误: {}", err),
            AppError::ParseError(err) => write!(f, "解析错误: {}", err),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::ParseError(error)
    }
}

pub type AppResult<T> = Result<T, AppError>;