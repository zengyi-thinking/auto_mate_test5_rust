//! 学生模块
//! 
//! 包含学生相关的所有功能

pub mod grade;
pub mod validator;

use crate::errors::{AppError, AppResult};
use self::grade::Grade;

#[derive(Debug, Clone)]
pub struct Student {
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub grades: Vec<Grade>,
}

impl Student {
    pub fn new(id: u32, name: String, age: u8) -> AppResult<Self> {
        validator::validate_age(age)?;
        validator::validate_name(&name)?;
        
        Ok(Student {
            id,
            name,
            age,
            grades: Vec::new(),
        })
    }
    
    pub fn add_grade(&mut self, subject: String, score: f64) -> AppResult<()> {
        let grade = Grade::new(subject, score)?;
        self.grades.push(grade);
        Ok(())
    }
    
    pub fn average_grade(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            let sum: f64 = self.grades.iter().map(|g| g.score()).sum();
            sum / self.grades.len() as f64
        }
    }
    
    pub fn get_grade_by_subject(&self, subject: &str) -> Option<&Grade> {
        self.grades.iter().find(|g| g.subject() == subject)
    }
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "学生[{}]: {} ({}岁)", self.id, self.name, self.age)
    }
}

use std::fmt;