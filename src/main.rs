//! Rust学习之旅 - 第3步：函数和控制流
//! 
//! 学习内容：
//! - 函数定义和参数
//! - 返回值和表达式
//! - 条件语句 if/else
//! - 循环：loop、while、for
//! - 模式匹配 match

fn main() {
    println!("🦀 Rust学习之旅 - 第3步：函数和控制流");
    println!("=".repeat(50));
    
    // 1. 函数基础
    demonstrate_functions();
    
    // 2. 条件语句
    demonstrate_conditions();
    
    // 3. 循环结构
    demonstrate_loops();
    
    // 4. 模式匹配
    demonstrate_pattern_matching();
}

/// 演示函数的定义和使用
fn demonstrate_functions() {
    println!("\n� 1. 函数基础");
    
    // 调用无参数函数
    greet();
    
    // 调用有参数函数
    let name = "小明";
    greet_person(name);
    
    // 调用有返回值的函数
    let sum = add_numbers(10, 20);
    println!("10 + 20 = {}", sum);
    
    // 表达式作为返回值
    let result = calculate_grade(87);
    println!("87分对应等级: {}", result);
    
    // 多个返回值（元组）
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
}

/// 无参数函数
fn greet() {
    println!("你好，欢迎学习Rust！");
}

/// 有参数的函数
fn greet_person(name: &str) {
    println!("你好，{}！", name);
}

/// 有返回值的函数
fn add_numbers(a: i32, b: i32) -> i32 {
    a + b  // 表达式，没有分号
}

/// 使用表达式返回值
fn calculate_grade(score: i32) -> char {
    if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    }
}

/// 返回多个值
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

/// 演示条件语句
fn demonstrate_conditions() {
    println!("\n🤔 2. 条件语句");
    
    let temperature = 25;
    
    // 基本 if-else
    if temperature > 30 {
        println!("天气很热！");
    } else if temperature > 20 {
        println!("天气很舒适。");
    } else {
        println!("天气有点凉。");
    }
    
    // if 作为表达式
    let weather_desc = if temperature > 25 { "温暖" } else { "凉爽" };
    println!("今天天气: {}", weather_desc);
    
    // 复杂条件
    let is_weekend = true;
    let has_homework = false;
    
    if is_weekend && !has_homework {
        println!("可以好好休息了！");
    } else if is_weekend && has_homework {
        println!("周末还要做作业...");
    } else {
        println!("工作日，继续努力！");
    }
}

/// 演示循环结构
fn demonstrate_loops() {
    println!("\n🔄 3. 循环结构");
    
    // for 循环遍历范围
    println!("倒计时:");
    for i in (1..=5).rev() {
        println!("  {}", i);
    }
    println!("  发射！🚀");
    
    // for 循环遍历数组
    let fruits = ["苹果", "香蕉", "橙子"];
    println!("\n水果清单:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }
    
    // while 循环
    println!("\n猜数字游戏模拟:");
    let target = 7;
    let mut guess = 1;
    while guess != target {
        println!("  猜测: {}", guess);
        guess += 2;
    }
    println!("  正确答案: {}！", target);
    
    // loop 循环（无限循环）
    println!("\n计算平方数:");
    let mut counter = 1;
    let result = loop {
        let square = counter * counter;
        println!("  {} 的平方是 {}", counter, square);
        
        if square > 20 {
            break square; // 从循环中返回值
        }
        counter += 1;
    };
    println!("第一个大于20的平方数: {}", result);
}

/// 演示模式匹配
fn demonstrate_pattern_matching() {
    println!("\n� 4. 模式匹配");
    
    // 基本 match
    let score = 85;
    match score {
        90..=100 => println!("优秀！"),
        80..=89 => println!("良好！"),
        70..=79 => println!("中等"),
        60..=69 => println!("及格"),
        _ => println!("不及格"),
    }
    
    // match 作为表达式
    let grade_point = match score {
        90..=100 => 4.0,
        80..=89 => 3.0,
        70..=79 => 2.0,
        60..=69 => 1.0,
        _ => 0.0,
    };
    println!("绩点: {:.1}", grade_point);
    
    // 匹配多个值
    let day = 3;
    let day_type = match day {
        1..=5 => "工作日",
        6 | 7 => "周末",
        _ => "无效日期",
    };
    println!("第{}天是: {}", day, day_type);
    
    // 匹配元组
    let point = (0, 5);
    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在Y轴上，y = {}", y),
        (x, 0) => println!("在X轴上，x = {}", x),
        (x, y) => println!("点({}, {})", x, y),
    }
    
    // 使用 if let 简化匹配
    let favorite_number = Some(7);
    if let Some(num) = favorite_number {
        println!("我最喜欢的数字是: {}", num);
    } else {
        println!("我没有最喜欢的数字");
    }
}
