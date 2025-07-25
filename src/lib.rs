//! Rust学习之旅 - 库文件
//! 
//! 这个文件定义了项目的公共API

pub mod student;
pub mod course;
pub mod utils;
pub mod errors;

// 重新导出常用类型
pub use student::Student;
pub use course::Course;
pub use errors::{AppError, AppResult};