//! 数学工具函数

use crate::errors::{AppError, AppResult};

pub fn calculate_gpa(grades: &[f64]) -> f64 {
    if grades.is_empty() {
        return 0.0;
    }
    
    let total_points: f64 = grades.iter()
        .map(|&grade| score_to_gpa(grade))
        .sum();
    
    total_points / grades.len() as f64
}

pub fn score_to_gpa(score: f64) -> f64 {
    match score {
        90.0..=100.0 => 4.0,
        80.0..=89.9 => 3.0,
        70.0..=79.9 => 2.0,
        60.0..=69.9 => 1.0,
        _ => 0.0,
    }
}

pub fn calculate_standard_deviation(values: &[f64]) -> AppResult<f64> {
    if values.is_empty() {
        return Err(AppError::InvalidInput(
            "不能计算空数组的标准差".to_string()
        ));
    }
    
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    
    Ok(variance.sqrt())
}

pub fn find_median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = values.len();
    
    if len % 2 == 0 {
        Some((values[len / 2 - 1] + values[len / 2]) / 2.0)
    } else {
        Some(values[len / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_score_to_gpa() {
        assert_eq!(score_to_gpa(95.0), 4.0);
        assert_eq!(score_to_gpa(85.0), 3.0);
        assert_eq!(score_to_gpa(75.0), 2.0);
        assert_eq!(score_to_gpa(65.0), 1.0);
        assert_eq!(score_to_gpa(55.0), 0.0);
    }
    
    #[test]
    fn test_calculate_gpa() {
        let grades = vec![90.0, 85.0, 75.0, 65.0];
        let gpa = calculate_gpa(&grades);
        assert_eq!(gpa, 2.5); // (4.0 + 3.0 + 2.0 + 1.0) / 4
    }
    
    #[test]
    fn test_find_median() {
        let mut values = vec![1.0, 3.0, 2.0, 5.0, 4.0];
        assert_eq!(find_median(&mut values), Some(3.0));
        
        let mut values = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(find_median(&mut values), Some(2.5));
    }
}