# Result<T, E> 和 ? 操作符学习笔记

## 1. Result<T, E> 基本概念

`Result<T, E>` 是Rust中用于**可恢复错误处理**的枚举类型：

```rust
enum Result<T, E> {
    Ok(T),   // 成功时包含值T
    Err(E),  // 失败时包含错误E
}
```

### 使用场景
- 文件操作（可能失败但程序不应崩溃）
- 网络请求（可能超时或连接失败）
- 数据解析（输入可能格式错误）
- 在线服务（一个用户错误不应影响其他用户）

## 2. Result的处理方式

### 2.1 用match处理（推荐）
```rust
let result = divide(10, 2);
match result {
    Ok(value) => println!("结果: {}", value),
    Err(error) => println!("错误: {}", error),
}
```

### 2.2 unwrap() - 简暴处理
```rust
let value = divide(10, 2).unwrap(); // 成功返回值，失败就panic
```

### 2.3 expect() - 带自定义错误信息
```rust
let value = divide(10, 2).expect("除法计算失败"); // 失败时显示自定义信息
```

## 3. ok_or() 方法

**作用**: 将 `Option<T>` 转换为 `Result<T, E>`

### 转换规则
```rust
Some(值) → Ok(值)
None → Err(你提供的错误信息)
```

### 实例
```rust
let account = find_account("0x123");        // 返回Option<u64>
let balance = account.ok_or("账户不存在");    // 转换为Result<u64, &str>

// 具体转换：
Some(1000) → Ok(1000)
None → Err("账户不存在")
```

## 4. ? 操作符

### 4.1 基本作用
**提前返回错误**，避免嵌套的match语句

### 4.2 处理规则
```rust
// 对于Result<T, E>：
Ok(值) → 取出值，继续执行
Err(错误) → 立即返回这个错误，停止执行

// 对于Option<T>：
Some(值) → 取出值，继续执行
None → 立即返回None，停止执行
```

### 4.3 使用示例
```rust
// 使用?操作符（简洁）
fn safe_transfer(from: &str, to: &str, amount: u64) -> Result<u64, String> {
    let from_balance = find_account(from).ok_or("发送方账户不存在")?;
    let _to_balance = find_account(to).ok_or("接收方账户不存在")?;
    
    if from_balance < amount {
        return Err("余额不足".to_string());
    }
    
    Ok(from_balance - amount)
}

// 等价于用match（啰嗦）
fn safe_transfer_verbose(from: &str, to: &str, amount: u64) -> Result<u64, String> {
    let from_balance = match find_account(from).ok_or("发送方账户不存在") {
        Ok(balance) => balance,
        Err(e) => return Err(e),
    };
    
    let _to_balance = match find_account(to).ok_or("接收方账户不存在") {
        Ok(balance) => balance,
        Err(e) => return Err(e),
    };
    
    if from_balance < amount {
        return Err("余额不足".to_string());
    }
    
    Ok(from_balance - amount)
}
```

## 5. ?操作符的限制

### 5.1 只能用于返回Result或Option的函数
```rust
// ✅ 正确
fn my_function() -> Result<i32, String> {
    let value = some_operation()?;
    Ok(value)
}

// ❌ 错误 - main函数默认返回()
fn main() {
    let value = some_operation()?; // 编译错误
}
```

### 5.2 需要变量承载正确的值
```rust
// ✅ 正确
let value = some_operation()?;

// ✅ 正确
some_operation()?.another_operation()?;

// ❌ 错误 - 缺少变量承载
some_operation()?; // 单独使用时需要分号但没有意义
```

### 5.3 ⚠️ 常见错误：?操作符会"解包"值
```rust
// ❌ 错误 - 类型不匹配
fn get_first_char(text: &str) -> Option<char> {
    text.chars().next()? // ?会提取出char，但函数要求返回Option<char>
}

// ✅ 正确方法1 - 直接返回
fn get_first_char(text: &str) -> Option<char> {
    text.chars().next()
}

// ✅ 正确方法2 - 使用?操作符
fn get_first_char(text: &str) -> Option<char> {
    let first_char = text.chars().next()?;  // 提取char
    Some(first_char)                        // 重新包装
}
```

## 6. 链式调用

?操作符支持链式调用，让代码更简洁：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

## 7. 错误传播

?操作符会**自动进行错误类型转换**，前提是实现了`From`特征：

```rust
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let f = File::open("hello.txt")?; // io::Error自动转换为Box<dyn Error>
    Ok(f)
}
```

## 8. 实际应用模式

### 8.1 Option → Result 转换
```rust
// 常见模式：查找可能不存在的资源
let user = find_user(id).ok_or("用户不存在")?;
let balance = get_balance(user).ok_or("余额查询失败")?;
```

### 8.2 多层错误处理
```rust
fn complex_operation() -> Result<String, String> {
    let data = fetch_data().ok_or("获取数据失败")?;
    let processed = process_data(data).ok_or("处理数据失败")?;
    let result = save_result(processed).ok_or("保存结果失败")?;
    Ok(result)
}
```

## 9. 最佳实践

1. **优先使用?操作符**而不是unwrap()
2. **用match处理需要不同逻辑的错误**
3. **用ok_or()将Option转换为Result**
4. **设计函数时考虑错误传播**
5. **在库代码中返回Result，让调用者决定如何处理错误**

## 10. 总结

- **Result<T, E>**: 可恢复错误处理的核心类型
- **ok_or()**: Option → Result 的桥梁
- **? 操作符**: 错误传播的简洁语法
- **match**: 精确控制错误处理逻辑
- **unwrap/expect**: 原型开发或确信不会失败的场景

这些概念在Solana开发中每天都会用到，特别是处理账户查找、余额检查、交易验证等场景。