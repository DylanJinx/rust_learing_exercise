fn main() {
    // 演示为什么不能索引字符串
    println!("=== 为什么 Rust 不允许字符串索引 ===\n");

    // 1. 不同语言的字符占用不同字节数
    let english = String::from("hello");
    let chinese = String::from("你好");
    let emoji = String::from("🦀🚀");

    println!("字符串及其字节表示:");
    println!("英文 '{}': {:?}", english, english.as_bytes());
    println!("中文 '{}': {:?}", chinese, chinese.as_bytes());
    println!("emoji '{}': {:?}", emoji, emoji.as_bytes());
    println!();

    // 2. 演示正确的字符访问方法
    println!("=== 正确的字符访问方法 ===\n");

    let mixed = String::from("Hi你好🦀");

    // 方法1: 遍历字符
    println!("逐个字符遍历:");
    for (i, c) in mixed.chars().enumerate() {
        println!("  字符 {}: '{}' (Unicode: U+{:04X})", i, c, c as u32);
    }
    println!();

    // 方法2: 获取特定位置的字符
    println!("获取特定位置字符:");
    if let Some(first) = mixed.chars().nth(0) {
        println!("  第0个字符: '{}'", first);
    }
    if let Some(third) = mixed.chars().nth(2) {
        println!("  第2个字符: '{}'", third);
    }
    println!();

    // 3. 字符串切片 (按字节边界)
    println!("=== 字符串切片 (需要小心字节边界) ===\n");

    let s = String::from("Hello世界");
    println!("原字符串: '{}'", s);
    println!("字节表示: {:?}", s.as_bytes());

    // 安全的切片
    let slice1 = &s[0..5]; // "Hello" - 5个ASCII字符
    let slice2 = &s[5..8]; // "世" - 3个字节的中文字符
    let slice3 = &s[8..11]; // "界" - 3个字节的中文字符

    println!("切片 [0..5]: '{}'", slice1);
    println!("切片 [5..8]: '{}'", slice2);
    println!("切片 [8..11]: '{}'", slice3);
    println!();

    // 4. 实用函数示例
    println!("=== 实用函数示例 ===\n");

    // 获取字符串长度的不同方式
    let text = String::from("Rust🦀中文");
    println!("字符串: '{}'", text);
    println!("字节长度: {}", text.len()); // 字节数
    println!("字符数量: {}", text.chars().count()); // Unicode字符数
    println!();

    // 安全的字符获取函数
    println!("使用安全的字符获取:");
    for i in 0..5 {
        match get_char_at(&text, i) {
            Some(c) => println!("  位置 {}: '{}'", i, c),
            None => println!("  位置 {}: 超出范围", i),
        }
    }
}

// 安全的字符获取函数
fn get_char_at(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

// 获取字符串前n个字符的安全函数
fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_access() {
        let s = "Hello世界";
        assert_eq!(get_char_at(s, 0), Some('H'));
        assert_eq!(get_char_at(s, 5), Some('世'));
        assert_eq!(get_char_at(s, 10), None);
    }

    #[test]
    fn test_take_chars() {
        let s = "Hello世界🦀";
        assert_eq!(take_chars(s, 3), "Hel");
        assert_eq!(take_chars(s, 6), "Hello世");
    }
}
