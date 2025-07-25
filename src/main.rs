//! Rust学习之旅 - 第6步：结构体
//! 
//! 学习内容：
//! - 结构体定义和实例化
//! - 方法和关联函数
//! - 结构体更新语法
//! - 元组结构体和单元结构体

// 定义学生结构体
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
    grade: f64,
    is_active: bool,
}

// 定义课程结构体
#[derive(Debug)]
struct Course {
    name: String,
    credits: u8,
    instructor: String,
}

// 元组结构体
#[derive(Debug)]
struct Point(i32, i32, i32);

// 单元结构体
#[derive(Debug)]
struct Unit;

fn main() {
    println!("🦀 Rust学习之旅 - 第6步：结构体");
    println!("=".repeat(50));
    
    // 1. 结构体基础
    demonstrate_struct_basics();
    
    // 2. 方法和关联函数
    demonstrate_methods();
    
    // 3. 结构体更新语法
    demonstrate_struct_update();
    
    // 4. 特殊结构体类型
    demonstrate_special_structs();
}

/// 演示结构体基础用法
fn demonstrate_struct_basics() {
    println!("\n🏗️ 1. 结构体基础");
    
    // 创建结构体实例
    let student1 = Student {
        name: String::from("张三"),
        age: 20,
        grade: 85.5,
        is_active: true,
    };
    
    println!("学生信息: {:?}", student1);
    println!("姓名: {}", student1.name);
    println!("年龄: {}", student1.age);
    println!("成绩: {:.1}", student1.grade);
    
    // 可变结构体
    let mut student2 = Student {
        name: String::from("李四"),
        age: 19,
        grade: 78.0,
        is_active: false,
    };
    
    println!("\n修改前: {:?}", student2);
    student2.grade = 82.5;
    student2.is_active = true;
    println!("修改后: {:?}", student2);
    
    // 使用函数创建结构体
    let student3 = create_student(String::from("王五"), 21, 90.0);
    println!("通过函数创建: {:?}", student3);
}

/// 创建学生的辅助函数
fn create_student(name: String, age: u8, grade: f64) -> Student {
    Student {
        name,  // 字段初始化简写
        age,
        grade,
        is_active: true,
    }
}

/// 演示方法和关联函数
impl Student {
    // 关联函数（类似静态方法）
    fn new(name: String, age: u8) -> Student {
        Student {
            name,
            age,
            grade: 0.0,
            is_active: true,
        }
    }
    
    // 方法（需要self参数）
    fn display_info(&self) {
        println!("学生: {}, 年龄: {}, 成绩: {:.1}", 
                 self.name, self.age, self.grade);
    }
    
    fn is_passing(&self) -> bool {
        self.grade >= 60.0
    }
    
    fn update_grade(&mut self, new_grade: f64) {
        self.grade = new_grade;
        println!("{} 的成绩更新为: {:.1}", self.name, self.grade);
    }
    
    fn get_grade_level(&self) -> &str {
        match self.grade {
            90.0..=100.0 => "优秀",
            80.0..=89.9 => "良好",
            70.0..=79.9 => "中等",
            60.0..=69.9 => "及格",
            _ => "不及格",
        }
    }
}

fn demonstrate_methods() {
    println!("\n🔧 2. 方法和关联函数");
    
    // 使用关联函数创建实例
    let mut student = Student::new(String::from("赵六"), 22);
    student.display_info();
    
    // 调用方法
    println!("是否及格: {}", student.is_passing());
    
    // 修改数据
    student.update_grade(87.5);
    student.display_info();
    println!("等级: {}", student.get_grade_level());
    println!("现在是否及格: {}", student.is_passing());
}

/// 演示结构体更新语法
fn demonstrate_struct_update() {
    println!("\n🔄 3. 结构体更新语法");
    
    let student1 = Student {
        name: String::from("原学生"),
        age: 20,
        grade: 85.0,
        is_active: true,
    };
    
    println!("原学生: {:?}", student1);
    
    // 使用结构体更新语法创建新实例
    let student2 = Student {
        name: String::from("新学生"),
        grade: 92.0,
        ..student1  // 其余字段从student1复制
    };
    
    println!("新学生: {:?}", student2);
    // 注意：student1的name被移动了，但age和is_active被复制了
    // println!("{:?}", student1); // 这行会编译错误
    
    // 创建课程实例
    let course1 = Course {
        name: String::from("Rust编程"),
        credits: 3,
        instructor: String::from("张教授"),
    };
    
    let course2 = Course {
        instructor: String::from("李教授"),
        ..course1
    };
    
    println!("课程1: {:?}", course2);
}

/// 演示特殊结构体类型
fn demonstrate_special_structs() {
    println!("\n🎯 4. 特殊结构体类型");
    
    // 元组结构体
    let origin = Point(0, 0, 0);
    let point1 = Point(1, 2, 3);
    
    println!("原点: {:?}", origin);
    println!("点1: {:?}", point1);
    println!("点1的坐标: ({}, {}, {})", point1.0, point1.1, point1.2);
    
    // 单元结构体
    let unit = Unit;
    println!("单元结构体: {:?}", unit);
    
    // 计算两点距离
    let distance = calculate_distance(&origin, &point1);
    println!("两点距离: {:.2}", distance);
}

/// 为Point实现方法
impl Point {
    fn new(x: i32, y: i32, z: i32) -> Point {
        Point(x, y, z)
    }
    
    fn distance_from_origin(&self) -> f64 {
        ((self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f64).sqrt()
    }
}

/// 计算两点间距离
fn calculate_distance(p1: &Point, p2: &Point) -> f64 {
    let dx = (p2.0 - p1.0) as f64;
    let dy = (p2.1 - p1.1) as f64;
    let dz = (p2.2 - p1.2) as f64;
    (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
}