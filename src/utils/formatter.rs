//! 格式化工具

use crate::Student;

pub fn format_student_list(students: &[&Student]) -> String {
    if students.is_empty() {
        return "没有学生".to_string();
    }
    
    let mut result = String::from("学生列表:\n");
    for (index, student) in students.iter().enumerate() {
        result.push_str(&format!(
            "  {}. {} (ID: {}, 年龄: {}, 平均分: {:.1})\n",
            index + 1,
            student.name,
            student.id,
            student.age,
            student.average_grade()
        ));
    }
    result
}

pub fn format_grade_table(student: &Student) -> String {
    if student.grades.is_empty() {
        return format!("{} 还没有成绩记录", student.name);
    }
    
    let mut table = format!("{}的成绩单:\n", student.name);
    table.push_str("┌─────────────────┬──────┬──────┐\n");
    table.push_str("│      科目       │ 分数 │ 等级 │\n");
    table.push_str("├─────────────────┼──────┼──────┤\n");
    
    for grade in &student.grades {
        table.push_str(&format!(
            "│ {:15} │ {:4.1} │  {:2}  │\n",
            grade.subject(),
            grade.score(),
            grade.letter_grade()
        ));
    }
    
    table.push_str("└─────────────────┴──────┴──────┘\n");
    table.push_str(&format!("平均分: {:.1}", student.average_grade()));
    
    table
}

pub fn format_progress_bar(current: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return "█".repeat(width);
    }
    
    let progress = (current as f64 / total as f64) * width as f64;
    let filled = progress as usize;
    let empty = width - filled;
    
    format!("[{}{}] {}/{}", 
            "█".repeat(filled),
            "░".repeat(empty),
            current,
            total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::student::Student;
    
    #[test]
    fn test_format_progress_bar() {
        assert_eq!(format_progress_bar(5, 10, 10), "[█████░░░░░] 5/10");
        assert_eq!(format_progress_bar(0, 10, 10), "[░░░░░░░░░░] 0/10");
        assert_eq!(format_progress_bar(10, 10, 10), "[██████████] 10/10");
    }
}