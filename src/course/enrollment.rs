//! 课程注册管理模块

use crate::errors::{AppError, AppResult};
use crate::{Course, Student};
use std::collections::HashMap;

pub struct EnrollmentManager {
    courses: HashMap<u32, Course>,
    students: HashMap<u32, Student>,
}

impl EnrollmentManager {
    pub fn new() -> Self {
        EnrollmentManager {
            courses: HashMap::new(),
            students: HashMap::new(),
        }
    }
    
    pub fn add_course(&mut self, course: Course) -> AppResult<()> {
        if self.courses.contains_key(&course.id) {
            return Err(AppError::ValidationError(
                format!("课程ID {} 已存在", course.id)
            ));
        }
        
        self.courses.insert(course.id, course);
        Ok(())
    }
    
    pub fn add_student(&mut self, student: Student) -> AppResult<()> {
        if self.students.contains_key(&student.id) {
            return Err(AppError::ValidationError(
                format!("学生ID {} 已存在", student.id)
            ));
        }
        
        self.students.insert(student.id, student);
        Ok(())
    }
    
    pub fn enroll_student_in_course(&mut self, student_id: u32, course_id: u32) -> AppResult<()> {
        // 检查学生是否存在
        let student = self.students.get(&student_id)
            .ok_or_else(|| AppError::NotFound(
                format!("学生ID {} 不存在", student_id)
            ))?;
        
        // 检查课程是否存在
        let course = self.courses.get_mut(&course_id)
            .ok_or_else(|| AppError::NotFound(
                format!("课程ID {} 不存在", course_id)
            ))?;
        
        // 注册学生
        course.enroll_student(student.clone())?;
        
        println!("学生 {} 成功注册课程 {}", student.name, course.name);
        Ok(())
    }
    
    pub fn get_course(&self, course_id: u32) -> Option<&Course> {
        self.courses.get(&course_id)
    }
    
    pub fn get_student(&self, student_id: u32) -> Option<&Student> {
        self.students.get(&student_id)
    }
    
    pub fn list_all_courses(&self) -> Vec<&Course> {
        self.courses.values().collect()
    }
    
    pub fn list_all_students(&self) -> Vec<&Student> {
        self.students.values().collect()
    }
}

impl Default for EnrollmentManager {
    fn default() -> Self {
        Self::new()
    }
}