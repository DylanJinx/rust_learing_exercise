use std::fs;

fn main() {
    println!("=== Result<T, E> 和 ? 操作符学习 ===\n");

    // 1. 基本的Result用法
    println!("1. 基本Result用法:");
    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    match result1 {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(error) => println!("错误: {}", error),
    }

    // 2. unwrap和expect的使用
    println!("\n2. unwrap和expect:");
    let success = divide(20, 4).unwrap(); // 成功时取出值
    println!("20 / 4 = {}", success);

    let success2 = divide(15, 3).expect("除法计算失败"); // 带自定义错误信息
    println!("15 / 3 = {}", success2);

    // 3. Option和ok_or的转换
    println!("\n3. Option转换为Result:");
    let account1 = find_account("0x1234567890");
    let account2 = find_account("不存在的地址");

    println!("账户1: {:?}", account1);
    println!("账户2: {:?}", account2);

    // 使用ok_or转换
    let balance1 = find_account("0x1234567890").ok_or("账户不存在");
    let balance2 = find_account("不存在的地址").ok_or("账户不存在");

    println!("转换后的余额1: {:?}", balance1);
    println!("转换后的余额2: {:?}", balance2);

    // 4. ?操作符的使用
    println!("\n4. ?操作符:");
    let transfer1 = safe_transfer("0x1234567890", "0x1234567891", 50);
    let transfer2 = safe_transfer("不存在", "0x1234567891", 50);

    println!("转账1结果: {:?}", transfer1);
    println!("转账2结果: {:?}", transfer2);

    // 5. 链式调用
    println!("\n5. 链式调用:");
    let chain_result = complex_operation("0x1234567890", 30);
    println!("复杂操作结果: {:?}", chain_result);

    // 6. 错误传播示例
    println!("\n6. 错误传播:");
    let file_content = read_file_content("test.txt");
    match file_content {
        Ok(content) => println!("文件内容: {}", content),
        Err(error) => println!("读取文件失败: {}", error),
    }

    // 7. ?操作符用于Option的正确用法
    println!("\n7. ?操作符用于Option:");
    let text = "Hello";
    let first_char = get_first_char(text);
    let second_char = get_second_char(text);
    let first_char_with_q = get_first_char_with_question_mark(text);

    println!("文本: '{}'", text);
    println!("第一个字符: {:?}", first_char);
    println!("第二个字符: {:?}", second_char);
    println!("第一个字符(用?): {:?}", first_char_with_q);

    // 空字符串测试
    let empty_text = "";
    println!("空字符串的第一个字符: {:?}", get_first_char(empty_text));
    println!("空字符串的第二个字符: {:?}", get_second_char(empty_text));
}

// 1. 基本的Result函数
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("除数不能为零".to_string())
    } else {
        Ok(a / b)
    }
}

// 2. 返回Option的函数（模拟账户查找）
fn find_account(address: &str) -> Option<u64> {
    match address {
        "0x1234567890" => Some(1000),
        "0x1234567891" => Some(500),
        "0x1234567892" => Some(800),
        _ => None,
    }
}

// 3. 使用?操作符的函数
fn safe_transfer(from: &str, to: &str, amount: u64) -> Result<u64, String> {
    // 使用?操作符处理Option到Result的转换
    let from_balance = find_account(from).ok_or("发送方账户不存在")?;
    let _to_balance = find_account(to).ok_or("接收方账户不存在")?;

    // 检查余额
    if from_balance < amount {
        return Err("余额不足".to_string());
    }

    // 返回转账后的余额
    Ok(from_balance - amount)
}

// 4. 链式调用示例
fn complex_operation(address: &str, amount: u64) -> Result<String, String> {
    let balance = find_account(address).ok_or("账户不存在")?;

    // 链式调用：先检查余额，再执行转账
    if balance >= amount {
        let remaining = balance - amount;
        Ok(format!("操作成功，剩余余额: {}", remaining))
    } else {
        Err("余额不足".to_string())
    }
}

// 5. 文件操作示例（展示真实的IO错误处理）
fn read_file_content(filename: &str) -> Result<String, String> {
    // 尝试读取文件，如果失败则返回错误
    match fs::read_to_string(filename) {
        Ok(content) => Ok(content),
        Err(error) => Err(format!("读取文件失败: {}", error)),
    }
}

// 6. 使用?操作符的文件操作
fn _read_file_with_question_mark(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(filename)?; // 直接传播错误
    Ok(content)
}

// 7. ?操作符用于Option
fn get_first_char(text: &str) -> Option<char> {
    text.chars().next() // 直接返回Option<char>
}

fn get_second_char(text: &str) -> Option<char> {
    let mut chars = text.chars();
    chars.next()?; // 跳过第一个字符，如果没有则返回None
    chars.next() // 返回第二个字符
}

// 正确使用?操作符的例子
fn get_first_char_with_question_mark(text: &str) -> Option<char> {
    let first_char = text.chars().next()?; // 提取char
    Some(first_char) // 包装回Option
}
