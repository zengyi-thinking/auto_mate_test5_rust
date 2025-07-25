//! Rust学习之旅 - 第10步：集合类型
//! 
//! 学习内容：
//! - Vector动态数组
//! - HashMap哈希映射
//! - 字符串处理
//! - 迭代器和闭包

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    grades: Vec<f64>,
}

impl Student {
    fn new(id: u32, name: String, age: u8) -> Self {
        Student {
            id,
            name,
            age,
            grades: Vec::new(),
        }
    }
    
    fn add_grade(&mut self, grade: f64) {
        self.grades.push(grade);
    }
    
    fn average_grade(&self) -> f64 {
        if self.grades.is_empty() {
            0.0
        } else {
            self.grades.iter().sum::<f64>() / self.grades.len() as f64
        }
    }
}

fn main() {
    println!("🦀 Rust学习之旅 - 第10步：集合类型");
    println!("=".repeat(50));
    
    // 1. Vector动态数组
    demonstrate_vectors();
    
    // 2. HashMap哈希映射
    demonstrate_hashmaps();
    
    // 3. 字符串处理
    demonstrate_strings();
    
    // 4. 迭代器和闭包
    demonstrate_iterators();
    
    // 5. 综合应用：学生管理系统
    demonstrate_student_system();
}

/// 演示Vector的使用
fn demonstrate_vectors() {
    println!("\n� 1. Vector动态数组");
    
    // 创建Vector
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("动态添加: {:?}", numbers);
    
    // 使用宏创建
    let fruits = vec!["苹果", "香蕉", "橙子", "葡萄"];
    println!("水果列表: {:?}", fruits);
    
    // 访问元素
    println!("第一个水果: {}", fruits[0]);
    match fruits.get(10) {
        Some(fruit) => println!("索引10: {}", fruit),
        None => println!("索引10超出范围"),
    }
    
    // 修改Vector
    let mut scores = vec![85, 92, 78, 96, 88];
    println!("原始分数: {:?}", scores);
    
    scores.push(94);
    println!("添加分数后: {:?}", scores);
    
    if let Some(last) = scores.pop() {
        println!("移除的分数: {}", last);
    }
    println!("移除后: {:?}", scores);
    
    // 遍历Vector
    println!("遍历分数:");
    for (index, score) in scores.iter().enumerate() {
        println!("  第{}个: {}", index + 1, score);
    }
    
    // 修改元素
    for score in &mut scores {
        *score += 5;  // 每个分数加5分
    }
    println!("加分后: {:?}", scores);
    
    // Vector的容量
    let mut capacity_demo = Vec::with_capacity(10);
    println!("初始容量: {}, 长度: {}", capacity_demo.capacity(), capacity_demo.len());
    
    for i in 0..15 {
        capacity_demo.push(i);
    }
    println!("添加15个元素后 - 容量: {}, 长度: {}", 
             capacity_demo.capacity(), capacity_demo.len());
}

/// 演示HashMap的使用
fn demonstrate_hashmaps() {
    println!("\n�️ 2. HashMap哈希映射");
    
    // 创建HashMap
    let mut student_grades = HashMap::new();
    student_grades.insert("张三", 85);
    student_grades.insert("李四", 92);
    student_grades.insert("王五", 78);
    
    println!("学生成绩: {:?}", student_grades);
    
    // 访问值
    match student_grades.get("张三") {
        Some(grade) => println!("张三的成绩: {}", grade),
        None => println!("未找到张三的成绩"),
    }
    
    // 插入或更新
    student_grades.insert("张三", 90);  // 更新
    student_grades.insert("赵六", 88);  // 插入
    println!("更新后: {:?}", student_grades);
    
    // 只在键不存在时插入
    student_grades.entry("孙七").or_insert(85);
    student_grades.entry("张三").or_insert(95);  // 不会覆盖
    println!("使用entry后: {:?}", student_grades);
    
    // 基于旧值更新
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("单词计数: {:?}", word_count);
    
    // 遍历HashMap
    println!("所有学生成绩:");
    for (name, grade) in &student_grades {
        println!("  {}: {}", name, grade);
    }
    
    // 从Vector创建HashMap
    let students = vec!["小明", "小红", "小刚"];
    let grades = vec![88, 92, 85];
    let grade_map: HashMap<_, _> = students.iter().zip(grades.iter()).collect();
    println!("从Vector创建: {:?}", grade_map);
}

/// 演示字符串处理
fn demonstrate_strings() {
    println!("\n📝 3. 字符串处理");
    
    // 创建字符串
    let mut greeting = String::new();
    greeting.push_str("你好");
    greeting.push('!');
    println!("构建的字符串: {}", greeting);
    
    // 字符串连接
    let hello = String::from("Hello");
    let world = String::from("World");
    let combined = hello + " " + &world;  // hello被移动了
    println!("连接字符串: {}", combined);
    
    // 使用format!宏
    let name = "Rust";
    let version = "1.70";
    let info = format!("{} 版本 {}", name, version);
    println!("格式化字符串: {}", info);
    
    // 字符串切片和索引
    let text = "Hello, 世界!";
    println!("原文: {}", text);
    println!("前5个字节: {}", &text[0..5]);
    // println!("{}", &text[0..8]); // 这会panic，因为切割了UTF-8字符
    
    // 安全的字符串操作
    let chinese = "你好世界";
    println!("中文字符串: {}", chinese);
    println!("字节长度: {}", chinese.len());
    println!("字符数量: {}", chinese.chars().count());
    
    // 遍历字符
    println!("逐个字符:");
    for c in chinese.chars() {
        println!("  {}", c);
    }
    
    // 字符串方法
    let sentence = "  Rust is awesome!  ";
    println!("原句: '{}'", sentence);
    println!("去空格: '{}'", sentence.trim());
    println!("转大写: '{}'", sentence.trim().to_uppercase());
    println!("包含'Rust': {}", sentence.contains("Rust"));
    println!("替换: '{}'", sentence.replace("awesome", "fantastic"));
    
    // 分割字符串
    let data = "apple,banana,orange,grape";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("分割结果: {:?}", fruits);
    
    // 字符串解析
    let numbers_str = "1,2,3,4,5";
    let numbers: Result<Vec<i32>, _> = numbers_str
        .split(',')
        .map(|s| s.parse())
        .collect();
    
    match numbers {
        Ok(nums) => println!("解析的数字: {:?}", nums),
        Err(e) => println!("解析错误: {}", e),
    }
}

/// 演示迭代器和闭包
fn demonstrate_iterators() {
    println!("\n🔄 4. 迭代器和闭包");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("原始数据: {:?}", numbers);
    
    // 基本迭代器操作
    let sum: i32 = numbers.iter().sum();
    println!("总和: {}", sum);
    
    let count = numbers.iter().count();
    println!("元素个数: {}", count);
    
    // 过滤和映射
    let even_squares: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)  // 过滤偶数
        .map(|&x| x * x)           // 计算平方
        .collect();
    println!("偶数的平方: {:?}", even_squares);
    
    // 查找操作
    let first_big = numbers.iter().find(|&&x| x > 5);
    match first_big {
        Some(num) => println!("第一个大于5的数: {}", num),
        None => println!("没有找到大于5的数"),
    }
    
    // 任意和全部
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("包含偶数: {}, 全部为正: {}", has_even, all_positive);
    
    // 闭包捕获环境
    let threshold = 5;
    let above_threshold: Vec<&i32> = numbers
        .iter()
        .filter(|

