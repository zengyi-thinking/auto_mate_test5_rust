//! 成绩模块
//! 
//! 处理学生成绩相关功能

use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct Grade {
    subject: String,
    score: f64,
    letter_grade: LetterGrade,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LetterGrade {
    A,  // 90-100
    B,  // 80-89
    C,  // 70-79
    D,  // 60-69
    F,  // 0-59
}

impl Grade {
    pub fn new(subject: String, score: f64) -> AppResult<Self> {
        if score < 0.0 || score > 100.0 {
            return Err(AppError::ValidationError(
                format!("成绩 {} 必须在0-100之间", score)
            ));
        }
        
        if subject.trim().is_empty() {
            return Err(AppError::ValidationError(
                "科目名称不能为空".to_string()
            ));
        }
        
        let letter_grade = LetterGrade::from_score(score);
        
        Ok(Grade {
            subject,
            score,
            letter_grade,
        })
    }
    
    pub fn subject(&self) -> &str {
        &self.subject
    }
    
    pub fn score(&self) -> f64 {
        self.score
    }
    
    pub fn letter_grade(&self) -> &LetterGrade {
        &self.letter_grade
    }
    
    pub fn is_passing(&self) -> bool {
        self.score >= 60.0
    }
}

impl LetterGrade {
    fn from_score(score: f64) -> Self {
        match score {
            90.0..=100.0 => LetterGrade::A,
            80.0..=89.9 => LetterGrade::B,
            70.0..=79.9 => LetterGrade::C,
            60.0..=69.9 => LetterGrade::D,
            _ => LetterGrade::F,
        }
    }
    
    pub fn to_gpa(&self) -> f64 {
        match self {
            LetterGrade::A => 4.0,
            LetterGrade::B => 3.0,
            LetterGrade::C => 2.0,
            LetterGrade::D => 1.0,
            LetterGrade::F => 0.0,
        }
    }
}

impl std::fmt::Display for LetterGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let letter = match self {
            LetterGrade::A => "A",
            LetterGrade::B => "B",
            LetterGrade::C => "C",
            LetterGrade::D => "D",
            LetterGrade::F => "F",
        };
        write!(f, "{}", letter)
    }
}

impl std::fmt::Display for Grade {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:.1} ({})", self.subject, self.score, self.letter_grade)
    }
}