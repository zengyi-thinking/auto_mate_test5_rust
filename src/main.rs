//! Rust学习之旅 - 第5步：引用和借用
//! 
//! 学习内容：
//! - 不可变引用
//! - 可变引用
//! - 借用规则
//! - 悬垂引用

fn main() {
    println!("🦀 Rust学习之旅 - 第5步：引用和借用");
    println!("=".repeat(50));
    
    // 1. 不可变引用
    demonstrate_immutable_references();
    
    // 2. 可变引用
    demonstrate_mutable_references();
    
    // 3. 借用规则
    demonstrate_borrowing_rules();
    
    // 4. 字符串切片
    demonstrate_string_slices();
}

/// 演示不可变引用
fn demonstrate_immutable_references() {
    println!("\n👀 1. 不可变引用");
    
    let message = String::from("Hello, Rust!");
    let len = calculate_length(&message);  // 传递引用，不转移所有权
    
    println!("字符串 '{}' 的长度是 {}", message, len);
    println!("原始字符串仍然可用: {}", message);
    
    // 多个不可变引用是允许的
    let ref1 = &message;
    let ref2 = &message;
    let ref3 = &message;
    
    println!("多个不可变引用:");
    println!("  ref1: {}", ref1);
    println!("  ref2: {}", ref2);
    println!("  ref3: {}", ref3);
}

/// 计算字符串长度（使用引用）
fn calculate_length(s: &String) -> usize {
    s.len()
} // s离开作用域，但因为它不拥有所指向的值，所以什么也不会发生

/// 演示可变引用
fn demonstrate_mutable_references() {
    println!("\n✏️ 2. 可变引用");
    
    let mut text = String::from("Hello");
    println!("修改前: {}", text);
    
    // 创建可变引用并修改
    append_world(&mut text);
    println!("修改后: {}", text);
    
    // 可变引用的作用域
    {
        let mutable_ref = &mut text;
        mutable_ref.push_str("!!!");
        println!("在作用域内修改: {}", mutable_ref);
    } // mutable_ref 在这里离开作用域
    
    // 现在可以再次使用text
    println!("最终结果: {}", text);
}

/// 向字符串追加内容
fn append_world(s: &mut String) {
    s.push_str(", World");
}

/// 演示借用规则
fn demonstrate_borrowing_rules() {
    println!("\n📏 3. 借用规则");
    
    let mut data = String::from("数据");
    
    // 规则1: 可以有多个不可变引用
    println!("规则1: 多个不可变引用");
    let r1 = &data;
    let r2 = &data;
    println!("  r1: {}, r2: {}", r1, r2);
    // r1 和 r2 在这里不再使用
    
    // 规则2: 只能有一个可变引用
    println!("规则2: 只能有一个可变引用");
    let r3 = &mut data;
    r3.push_str("修改");
    println!("  r3: {}", r3);
    // r3 在这里不再使用
    
    // 规则3: 不能同时有可变和不可变引用
    println!("规则3: 不能同时有可变和不可变引用");
    let r4 = &data;  // 不可变引用
    println!("  r4: {}", r4);
    // 在r4使用完之后，才能创建可变引用
    
    let r5 = &mut data;  // 可变引用
    r5.push_str("!");
    println!("  r5: {}", r5);
    
    println!("最终数据: {}", data);
}

/// 演示字符串切片
fn demonstrate_string_slices() {
    println!("\n🔪 4. 字符串切片");
    
    let sentence = String::from("Hello Rust Programming");
    
    // 字符串切片
    let hello = &sentence[0..5];
    let rust = &sentence[6..10];
    let programming = &sentence[11..];
    
    println!("原句: {}", sentence);
    println!("切片:");
    println!("  hello: {}", hello);
    println!("  rust: {}", rust);
    println!("  programming: {}", programming);
    
    // 获取第一个单词
    let first_word = get_first_word(&sentence);
    println!("第一个单词: {}", first_word);
    
    // 字符串字面量就是切片
    let literal = "这是字符串字面量";  // 类型是 &str
    let first_word_literal = get_first_word(literal);
    println!("字面量的第一个词: {}", first_word_literal);
    
    // 数组切片
    let numbers = [1, 2, 3, 4, 5, 6];
    let slice = &numbers[1..4];
    println!("数组: {:?}", numbers);
    println!("切片 [1..4]: {:?}", slice);
}

/// 获取字符串的第一个单词
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]  // 如果没有空格，返回整个字符串
}