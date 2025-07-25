//! 课程统计模块

use crate::Course;

pub struct CourseStatistics {
    pub average_grade: f64,
    pub highest_grade: f64,
    pub lowest_grade: f64,
    pub passing_rate: f64,
    pub grade_distribution: GradeDistribution,
}

#[derive(Debug)]
pub struct GradeDistribution {
    pub a_count: usize,
    pub b_count: usize,
    pub c_count: usize,
    pub d_count: usize,
    pub f_count: usize,
}

impl Course {
    pub fn calculate_statistics(&self) -> Option<CourseStatistics> {
        if self.students.is_empty() {
            return None;
        }
        
        let averages: Vec<f64> = self.students.values()
            .map(|s| s.average_grade())
            .collect();
        
        if averages.is_empty() {
            return None;
        }
        
        let total_average = averages.iter().sum::<f64>() / averages.len() as f64;
        let highest = averages.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let lowest = averages.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        
        let passing_count = averages.iter().filter(|&&avg| avg >= 60.0).count();
        let passing_rate = (passing_count as f64 / averages.len() as f64) * 100.0;
        
        let grade_distribution = calculate_grade_distribution(&averages);
        
        Some(CourseStatistics {
            average_grade: total_average,
            highest_grade: highest,
            lowest_grade: lowest,
            passing_rate,
            grade_distribution,
        })
    }
}

fn calculate_grade_distribution(averages: &[f64]) -> GradeDistribution {
    let mut distribution = GradeDistribution {
        a_count: 0,
        b_count: 0,
        c_count: 0,
        d_count: 0,
        f_count: 0,
    };
    
    for &avg in averages {
        match avg {
            90.0..=100.0 => distribution.a_count += 1,
            80.0..=89.9 => distribution.b_count += 1,
            70.0..=79.9 => distribution.c_count += 1,
            60.0..=69.9 => distribution.d_count += 1,
            _ => distribution.f_count += 1,
        }
    }
    
    distribution
}

impl std::fmt::Display for CourseStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "课程统计:")?;
        writeln!(f, "  平均分: {:.1}", self.average_grade)?;
        writeln!(f, "  最高分: {:.1}", self.highest_grade)?;
        writeln!(f, "  最低分: {:.1}", self.lowest_grade)?;
        writeln!(f, "  及格率: {:.1}%", self.passing_rate)?;
        writeln!(f, "  成绩分布:")?;
        writeln!(f, "    A: {} 人", self.grade_distribution.a_count)?;
        writeln!(f, "    B: {} 人", self.grade_distribution.b_count)?;
        writeln!(f, "    C: {} 人", self.grade_distribution.c_count)?;
        writeln!(f, "    D: {} 人", self.grade_distribution.d_count)?;
        write!(f, "    F: {} 人", self.grade_distribution.f_count)
    }
}