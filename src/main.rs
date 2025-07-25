//! Rust学习之旅 - 第4步：所有权基础
//! 
//! 学习内容：
//! - 所有权规则
//! - 移动语义
//! - 克隆数据
//! - 栈和堆的区别

fn main() {
    println!("🦀 Rust学习之旅 - 第4步：所有权基础");
    println!("=".repeat(50));
    
    // 1. 所有权规则
    demonstrate_ownership_rules();
    
    // 2. 移动语义
    demonstrate_move_semantics();
    
    // 3. 克隆数据
    demonstrate_cloning();
    
    // 4. 栈和堆
    demonstrate_stack_and_heap();
}

/// 演示所有权的基本规则
fn demonstrate_ownership_rules() {
    println!("\n📋 1. 所有权规则");
    println!("规则1: Rust中每个值都有一个所有者");
    println!("规则2: 同一时间只能有一个所有者");
    println!("规则3: 当所有者离开作用域时，值被丢弃");
    
    {
        let message = String::from("Hello, Rust!");  // message 是所有者
        println!("在作用域内: {}", message);
    } // message 在这里离开作用域并被丢弃
    
    // println!("{}", message); // 这行会编译错误，message已不存在
    
    println!("演示完成：变量在作用域结束时自动清理");
}

/// 演示移动语义
fn demonstrate_move_semantics() {
    println!("\n📦 2. 移动语义");
    
    // 栈上数据的复制
    let x = 5;
    let y = x;  // 复制，因为i32实现了Copy trait
    println!("栈数据复制: x = {}, y = {}", x, y);
    
    // 堆上数据的移动
    let s1 = String::from("学习Rust");
    println!("原始字符串: {}", s1);
    
    let s2 = s1;  // 移动！s1不再有效
    println!("移动后: {}", s2);
    // println!("{}", s1); // 这行会编译错误
    
    // 函数调用也会发生移动
    let s3 = String::from("函数调用");
    println!("调用前: {}", s3);
    take_ownership(s3);  // s3的所有权移动到函数中
    // println!("{}", s3); // 这行会编译错误
    
    // 函数返回值可以转移所有权
    let s4 = give_ownership();
    println!("从函数获得: {}", s4);
    
    let s5 = String::from("传递并返回");
    let s6 = take_and_give_back(s5);
    println!("传递并返回: {}", s6);
}

/// 接受所有权的函数
fn take_ownership(some_string: String) {
    println!("函数内部: {}", some_string);
} // some_string在这里离开作用域并被丢弃

/// 返回所有权的函数
fn give_ownership() -> String {
    let some_string = String::from("返回的字符串");
    some_string  // 返回并移动所有权给调用者
}

/// 接受并返回所有权
fn take_and_give_back(a_string: String) -> String {
    a_string  // 返回并移动所有权给调用者
}

/// 演示克隆数据
fn demonstrate_cloning() {
    println!("\n🔄 3. 克隆数据");
    
    let s1 = String::from("原始数据");
    let s2 = s1.clone();  // 深拷贝
    
    println!("原始: {}", s1);
    println!("克隆: {}", s2);
    println!("两个变量都可以使用！");
    
    // 演示克隆的成本
    let large_string = "很长的字符串".repeat(1000);
    println!("大字符串长度: {}", large_string.len());
    
    let cloned_string = large_string.clone();
    println!("克隆完成，长度: {}", cloned_string.len());
    println!("注意：克隆大数据会有性能开销");
    
    // 实现Copy trait的类型会自动复制
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;  // 数组实现了Copy，所以这是复制不是移动
    println!("数组1: {:?}", arr1);
    println!("数组2: {:?}", arr2);
}

/// 演示栈和堆的区别
fn demonstrate_stack_and_heap() {
    println!("\n🏗️ 4. 栈和堆的区别");
    
    // 栈上的数据
    println!("栈上数据（固定大小，快速访问）:");
    let stack_number = 42;
    let stack_array = [1, 2, 3, 4, 5];
    let stack_tuple = (10, 20, 30);
    
    println!("  数字: {}", stack_number);
    println!("  数组: {:?}", stack_array);
    println!("  元组: {:?}", stack_tuple);
    
    // 堆上的数据
    println!("\n堆上数据（动态大小，需要分配）:");
    let heap_string = String::from("存储在堆上");
    let heap_vector = vec![1, 2, 3, 4, 5];
    
    println!("  字符串: {}", heap_string);
    println!("  向量: {:?}", heap_vector);
    
    // 展示String和&str的区别
    let string_literal = "字符串字面量";  // 存储在程序二进制中
    let string_object = String::from("字符串对象");  // 存储在堆上
    
    println!("\n字符串类型:");
    println!("  字面量(&str): {}", string_literal);
    println!("  对象(String): {}", string_object);
    
    // 演示容量和长度
    let mut dynamic_string = String::with_capacity(50);
    dynamic_string.push_str("动态增长的字符串");
    
    println!("\n动态字符串信息:");
    println!("  内容: {}", dynamic_string);
    println!("  长度: {}", dynamic_string.len());
    println!("  容量: {}", dynamic_string.capacity());
}