//! 课程模块

pub mod enrollment;
pub mod statistics;

use crate::errors::{AppError, AppResult};
use crate::student::Student;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Course {
    pub id: u32,
    pub name: String,
    pub credits: u8,
    pub instructor: String,
    students: HashMap<u32, Student>,
}

impl Course {
    pub fn new(id: u32, name: String, credits: u8, instructor: String) -> AppResult<Self> {
        if name.trim().is_empty() {
            return Err(AppError::ValidationError(
                "课程名称不能为空".to_string()
            ));
        }
        
        if credits == 0 || credits > 10 {
            return Err(AppError::ValidationError(
                "学分必须在1-10之间".to_string()
            ));
        }
        
        Ok(Course {
            id,
            name,
            credits,
            instructor,
            students: HashMap::new(),
        })
    }
    
    pub fn enroll_student(&mut self, student: Student) -> AppResult<()> {
        if self.students.contains_key(&student.id) {
            return Err(AppError::ValidationError(
                format!("学生 {} 已经注册了这门课程", student.name)
            ));
        }
        
        self.students.insert(student.id, student);
        Ok(())
    }
    
    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }
    
    pub fn get_student_mut(&mut self, student_id: u32) -> Option<&mut Student> {
        self.students.get_mut(&student_id)
    }
    
    pub fn remove_student(&mut self, student_id: u32) -> AppResult<Student> {
        self.students.remove(&student_id)
            .ok_or_else(|| AppError::NotFound(
                format!("学生ID {} 未找到", student_id)
            ))
    }
    
    pub fn student_count(&self) -> usize {
        self.students.len()
    }
    
    pub fn list_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }
}

impl std::fmt::Display for Course {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "课程[{}]: {} ({}学分, 教师: {}, 学生数: {})", 
               self.id, self.name, self.credits, self.instructor, self.student_count())
    }
}