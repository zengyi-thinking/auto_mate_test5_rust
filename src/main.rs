//! Rust学习之旅 - 第8步：错误处理
//! 
//! 学习内容：
//! - panic!宏和不可恢复错误
//! - Result类型和可恢复错误
//! - 错误传播和?操作符
//! - 自定义错误类型

use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

// 自定义错误类型
#[derive(Debug)]
enum StudentError {
    InvalidAge(String),
    InvalidGrade(String),
    NotFound(String),
    IoError(io::Error),
    ParseError(ParseIntError),
}

// 为自定义错误实现Display
impl std::fmt::Display for StudentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            StudentError::InvalidAge(msg) => write!(f, "年龄错误: {}", msg),
            StudentError::InvalidGrade(msg) => write!(f, "成绩错误: {}", msg),
            StudentError::NotFound(msg) => write!(f, "未找到: {}", msg),
            StudentError::IoError(err) => write!(f, "IO错误: {}", err),
            StudentError::ParseError(err) => write!(f, "解析错误: {}", err),
        }
    }
}

// 实现Error trait
impl std::error::Error for StudentError {}

// 从其他错误类型转换
impl From<io::Error> for StudentError {
    fn from(error: io::Error) -> Self {
        StudentError::IoError(error)
    }
}

impl From<ParseIntError> for StudentError {
    fn from(error: ParseIntError) -> Self {
        StudentError::ParseError(error)
    }
}

type StudentResult<T> = Result<T, StudentError>;

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grade: f64,
}

impl Student {
    fn new(name: String, age: u8, grade: f64) -> StudentResult<Student> {
        if age > 100 {
            return Err(StudentError::InvalidAge(
                format!("年龄 {} 超出合理范围", age)
            ));
        }
        
        if grade < 0.0 || grade > 100.0 {
            return Err(StudentError::InvalidGrade(
                format!("成绩 {} 不在0-100范围内", grade)
            ));
        }
        
        Ok(Student { name, age, grade })
    }
}

fn main() {
    println!("🦀 Rust学习之旅 - 第8步：错误处理");
    println!("=".repeat(50));
    
    // 1. panic和不可恢复错误
    demonstrate_panic();
    
    // 2. Result和可恢复错误
    demonstrate_result();
    
    // 3. 错误传播
    demonstrate_error_propagation();
    
    // 4. 自定义错误处理
    demonstrate_custom_errors();
    
    // 5. 错误处理最佳实践
    demonstrate_best_practices();
}

/// 演示panic和不可恢复错误
fn demonstrate_panic() {
    println!("\n💥 1. panic和不可恢复错误");
    
    // 数组越界会panic
    let numbers = vec![1, 2, 3];
    println!("数组: {:?}", numbers);
    
    // 安全访问
    match numbers.get(5) {
        Some(value) => println!("索引5的值: {}", value),
        None => println!("索引5超出范围"),
    }
    
    // 手动panic（在实际代码中谨慎使用）
    // panic!("这是一个手动panic!");
    
    println!("演示完成（跳过了实际panic调用）");
}

/// 演示Result和可恢复错误
fn demonstrate_result() {
    println!("\n🔧 2. Result和可恢复错误");
    
    // 字符串解析
    let number_strings = vec!["42", "abc", "123", "xyz", "789"];
    
    println!("解析数字:");
    for s in number_strings {
        match parse_number(s) {
            Ok(num) => println!("  '{}' -> {}", s, num),
            Err(e) => println!("  '{}' -> 错误: {}", s, e),
        }
    }
    
    // 除法运算
    let divisions = vec![(10, 2), (15, 3), (8, 0), (20, 4)];
    println!("\n除法运算:");
    for (a, b) in divisions {
        match safe_divide(a, b) {
            Ok(result) => println!("  {} ÷ {} = {}", a, b, result),
            Err(msg) => println!("  {} ÷ {} -> 错误: {}", a, b, msg),
        }
    }
}

fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>()
}

fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

/// 演示错误传播
fn demonstrate_error_propagation() {
    println!("\n📤 3. 错误传播");
    
    // 使用?操作符传播错误
    match read_and_parse_file() {
        Ok(numbers) => {
            println!("成功读取并解析文件:");
            println!("  数字: {:?}", numbers);
            println!("  总和: {}", numbers.iter().sum::<i32>());
        }
        Err(e) => println!("处理文件时出错: {}", e),
    }
    
    // 链式错误处理
    match process_student_data("25", "87.5") {
        Ok(student) => println!("创建学生成功: {:?}", student),
        Err(e) => println!("创建学生失败: {}", e),
    }
    
    match process_student_data("abc", "87.5") {
        Ok(student) => println!("创建学生成功: {:?}", student),
        Err(e) => println!("创建学生失败: {}", e),
    }
}

fn read_and_parse_file() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    // 模拟文件内容
    let content = "1\n2\n3\n4\n5";
    
    let mut numbers = Vec::new();
    for line in content.lines() {
        let number = line.parse::<i32>()?;  // ?操作符传播错误
        numbers.push(number);
    }
    
    Ok(numbers)
}

fn process_student_data(age_str: &str, grade_str: &str) -> StudentResult<Student> {
    let age: u8 = age_str.parse()?;  // 自动转换ParseIntError
    let grade: f64 = grade_str.parse().map_err(|_| {
        StudentError::InvalidGrade("无法解析成绩".to_string())
    })?;
    
    Student::new("测试学生".to_string(), age, grade)
}

/// 演示自定义错误处理
fn demonstrate_custom_errors() {
    println!("\n🎯 4. 自定义错误处理");
    
    let test_cases = vec![
        ("张三", 20, 85.0),
        ("李四", 150, 90.0),  // 年龄错误
        ("王五", 22, 105.0),  // 成绩错误
        ("赵六", 19, 78.5),
    ];
    
    println!("创建学生:");
    for (name, age, grade) in test_cases {
        match Student::new(name.to_string(), age, grade) {
            Ok(student) => println!("  ✓ 成功: {:?}", student),
            Err(e) => println!("  ✗ 失败: {}", e),
        }
    }
    
    // 查找学生
    let students = vec![
        Student::new("小明".to_string(), 20, 85.0).unwrap(),
        Student::new("小红".to_string(), 19, 92.0).unwrap(),
    ];
    
    println!("\n查找学生:");
    match find_student(&students, "小明") {
        Ok(student) => println!("  找到: {:?}", student),
        Err(e) => println!("  错误: {}", e),
    }
    
    match find_student(&students, "小刚") {
        Ok(student) => println!("  找到: {:?}", student),
        Err(e) => println!("  错误: {}", e),
    }
}

fn find_student(students: &[Student], name: &str) -> StudentResult<&Student> {
    students.iter()
        .find(|s| s.name == name)
        .ok_or_else(|| StudentError::NotFound(format!("学生 '{}' 不存在", name)))
}

/// 演示错误处理最佳实践
fn demonstrate_best_practices() {
    println!("\n⭐ 5. 错误处理最佳实践");
    
    // 使用unwrap_or提供默认值
    let input = "abc";
    let number = input.parse::<i32>().unwrap_or(0);
    println!("解析 '{}' 结果: {} (默认值)", input, number);
    
    // 使用unwrap_or_else提供计算的默认值
    let number2 = input.parse::<i32>().unwrap_or_else(|_| {
        println!("  解析失败，使用默认值");
        -1
    });
    println!("解析结果: {}", number2);
    
    // 使用map转换成功值
    let result = "42".parse::<i32>()
        .map(|n| n * 2)
        .unwrap_or(0);
    println!("解析并翻倍: {}", result);
    
    // 使用and_then链式操作
    let result2 = "10".parse::<i32>()
        .and_then(|n| safe_divide(n, 2).map_err(|e| e.parse().unwrap_or_default()))
        .unwrap_or(0);
    println!("解析并除以2: {}", result2);
    
    // 错误日志记录
    if let Err(e) = risky_operation() {
        eprintln!("操作失败: {}", e);
        println!("已记录错误到stderr");
    }
}

fn risky_operation() -> Result<(), String> {
    // 模拟可能失败的操作
    Err("模拟的错误".to_string())
}