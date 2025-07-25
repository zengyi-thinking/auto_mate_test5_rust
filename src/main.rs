//! Rust学习之旅 - 第2步：变量和数据类型
//! 
//! 学习内容：
//! - 变量的可变性和不可变性
//! - 基本数据类型
//! - 常量和静态变量
//! - 变量遮蔽

fn main() {
    println!("🦀 Rust学习之旅 - 第2步：变量和数据类型");
    println!("=".repeat(50));
    
    // 1. 变量声明和可变性
    demonstrate_variables();
    
    // 2. 基本数据类型
    demonstrate_data_types();
    
    // 3. 常量和静态变量
    demonstrate_constants();
    
    // 4. 变量遮蔽
    demonstrate_shadowing();
}

/// 演示变量的可变性
fn demonstrate_variables() {
    println!("\n📝 1. 变量的可变性");
    
    // 不可变变量（默认）
    let student_name = "张三";
    println!("学生姓名: {}", student_name);
    
    // 可变变量
    let mut score = 85;
    println!("初始分数: {}", score);
    
    score = score + 10;  // 修改可变变量
    println!("修改后分数: {}", score);
    
    // let student_name = "李四"; // 这会创建新变量，不是修改
}

/// 演示基本数据类型
fn demonstrate_data_types() {
    println!("\n🔢 2. 基本数据类型");
    
    // 整数类型
    let age: u8 = 20;
    let population: u32 = 1_400_000_000;
    println!("年龄: {}, 人口: {}", age, population);
    
    // 浮点类型
    let gpa: f64 = 3.85;
    let temperature: f32 = 36.5;
    println!("GPA: {:.2}, 体温: {}°C", gpa, temperature);
    
    // 布尔类型
    let is_student = true;
    let has_scholarship = false;
    println!("是学生: {}, 有奖学金: {}", is_student, has_scholarship);
    
    // 字符类型
    let grade = 'A';
    let emoji = '🎓';
    println!("等级: {}, 表情: {}", grade, emoji);
    
    // 复合类型：元组
    let student_info = ("王五", 22, 3.9);
    println!("学生信息: 姓名={}, 年龄={}, GPA={}", 
             student_info.0, student_info.1, student_info.2);
    
    // 复合类型：数组
    let grades = [88, 92, 76, 95, 89];
    println!("成绩数组: {:?}", grades);
    println!("第一门课成绩: {}", grades[0]);
    println!("数组长度: {}", grades.len());
}

/// 演示常量和静态变量
const MAX_SCORE: u32 = 100;
const UNIVERSITY_NAME: &str = "Rust大学";

static COURSE_COUNT: u32 = 5;

fn demonstrate_constants() {
    println!("\n📌 3. 常量和静态变量");
    
    println!("最高分数: {}", MAX_SCORE);
    println!("大学名称: {}", UNIVERSITY_NAME);
    println!("课程数量: {}", COURSE_COUNT);
    
    // 常量可以在任何作用域声明，包括全局作用域
    const PASSING_SCORE: u32 = 60;
    println!("及格分数: {}", PASSING_SCORE);
}

/// 演示变量遮蔽
fn demonstrate_shadowing() {
    println!("\n🎭 4. 变量遮蔽");
    
    let score = 85;
    println!("原始分数: {}", score);
    
    // 遮蔽：创建同名的新变量
    let score = score + 10;
    println!("加分后: {}", score);
    
    // 遮蔽可以改变类型
    let score = format!("{}分", score);
    println!("格式化后: {}", score);
    
    // 在新的作用域中遮蔽
    {
        let score = "优秀";
        println!("作用域内: {}", score);
    }
    
    println!("作用域外: {}", score);
}