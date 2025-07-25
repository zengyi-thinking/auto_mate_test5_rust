//! 学生数据验证模块

use crate::errors::{AppError, AppResult};

pub fn validate_age(age: u8) -> AppResult<()> {
    if age < 5 {
        return Err(AppError::ValidationError(
            format!("年龄 {} 太小，必须至少5岁", age)
        ));
    }
    
    if age > 100 {
        return Err(AppError::ValidationError(
            format!("年龄 {} 太大，必须小于100岁", age)
        ));
    }
    
    Ok(())
}

pub fn validate_name(name: &str) -> AppResult<()> {
    if name.trim().is_empty() {
        return Err(AppError::ValidationError(
            "姓名不能为空".to_string()
        ));
    }
    
    if name.len() > 50 {
        return Err(AppError::ValidationError(
            "姓名长度不能超过50个字符".to_string()
        ));
    }
    
    // 检查是否包含数字
    if name.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::ValidationError(
            "姓名不能包含数字".to_string()
        ));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_age() {
        assert!(validate_age(20).is_ok());
        assert!(validate_age(5).is_ok());
        assert!(validate_age(99).is_ok());
    }
    
    #[test]
    fn test_invalid_age() {
        assert!(validate_age(4).is_err());
        assert!(validate_age(101).is_err());
    }
    
    #[test]
    fn test_valid_name() {
        assert!(validate_name("张三").is_ok());
        assert!(validate_name("李四").is_ok());
    }
    
    #[test]
    fn test_invalid_name() {
        assert!(validate_name("").is_err());
        assert!(validate_name("张三123").is_err());
        assert!(validate_name(&"a".repeat(51)).is_err());
    }
}