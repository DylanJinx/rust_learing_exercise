# Rust 字符串类型详解

## 概述

本笔记总结了 Rust 中字符串相关类型的核心概念，包括 `&str`、`String`、`&mut str` 和 `&mut String` 的区别与使用。

## 1. 字符串字面值与 `&str`

### 基本概念

```rust
let s = "hello";  // s 的类型是 &str
```

### 内存布局

- **字符串数据**：存储在程序二进制文件的**只读数据段**（不是堆！）
- **`&str` 变量**：存储在栈上，包含：
  - 指向字符串数据的指针
  - 字符串长度信息

```rust
// 内存布局示意：
// 栈内存：
// s = { ptr: 0x12345678, len: 5 }
//           ↓
// 程序二进制文件（只读数据段）：
// 0x12345678: ['h', 'e', 'l', 'l', 'o']
```

### 重要特性

- `&str` 是不可变引用
- 字符串字面值具有 `'static` 生命周期
- 数据存储在只读内存区域，无法修改

## 2. 变量绑定 vs 数据修改

### 可以重新绑定

```rust
let mut s = "hello";  // 可变绑定
s = "world";          // ✅ 重新绑定到新的字符串字面值
println!("{}", s);    // 输出: world
```

**发生了什么：**

- `s` 从指向 `"hello"` 变为指向 `"world"`
- 没有修改任何字符串数据本身
- 两个字符串字面值都继续存在于只读数据段

### 不能修改数据内容

```rust
let mut s = "hello";
// s[0] = 'H';  // ❌ 编译错误！不能修改字符串内容
```

## 3. `&str` 的不同数据源

`&str` 可以指向不同位置的字符串数据：

```rust
fn main() {
    // 1. 指向程序二进制文件中的字符串字面值
    let s1: &str = "hello";

    // 2. 指向堆上 String 的数据
    let string = String::from("world");
    let s2: &str = &string;

    // 3. 指向其他内存位置的数据
    let arr = [104, 105]; // 'h', 'i' 的 ASCII 值
    let s3: &str = std::str::from_utf8(&arr).unwrap();

    // 验证数据地址不同
    println!("s1 数据地址: {:p}", s1.as_ptr());  // 只读数据段
    println!("s2 数据地址: {:p}", s2.as_ptr());  // 堆地址
    println!("s3 数据地址: {:p}", s3.as_ptr());  // 栈地址
}
```

## 4. String 类型

### 基本特性

```rust
let mut s = String::from("hello");
s.push_str(", world!");  // ✅ 可以修改内容和长度
```

### 内存布局

- **String 变量**：存储在栈上，包含：
  - 指向堆数据的指针
  - 长度 (len)
  - 容量 (capacity)
- **字符串数据**：存储在堆上，可以动态增长

## 5. 四种字符串引用类型对比

| 类型          | 可变性   | 能否改变长度 | 常用方法示例                    | 使用场景               |
| ------------- | -------- | ------------ | ------------------------------- | ---------------------- |
| `&str`        | 不可变   | ❌           | 读取、比较                      | 字符串字面值、只读引用 |
| `&mut str`    | 可变内容 | ❌           | `make_ascii_uppercase()`        | 就地修改字符           |
| `&String`     | 不可变   | ❌           | 同 `&str`                       | String 的只读引用      |
| `&mut String` | 完全可变 | ✅           | `push()`, `push_str()`, `pop()` | 完整的 String 操作     |

## 6. 实际代码示例

### `&mut str` 的正确使用

```rust
fn main() {
    let mut s = String::from("hello");

    // 获取 &mut str
    let s_mut_str: &mut str = s.as_mut_str();

    // 只能修改现有字符，不能改变长度
    s_mut_str.make_ascii_uppercase();
    println!("{}", s_mut_str); // 输出: HELLO

    // s_mut_str.push('!');  // ❌ 编译错误！&mut str 没有 push 方法
}
```

### `&mut String` 的使用

```rust
fn main() {
    let mut s = String::from("hello");

    // 获取 &mut String
    let s_mut_string: &mut String = &mut s;

    // 可以完整操作 String
    s_mut_string.push('!');           // ✅ 添加字符
    s_mut_string.push_str(" world");  // ✅ 添加字符串
    s_mut_string.pop();               // ✅ 删除字符

    println!("{}", s_mut_string);
}
```

### 类型验证

```rust
fn main() {
    let mut s = String::from("hello");
    let s_mut = &mut s;  // 类型是 &mut String

    // 验证类型
    println!("类型: {}", std::any::type_name_of_val(&s_mut));
    // 输出: &mut alloc::string::String

    s_mut.push('!');  // String 的方法
}
```

## 7. 自动解引用 (Deref Coercion)

```rust
fn main() {
    let mut s = String::from("hello");
    let s_mut = &mut s;  // &mut String

    // 由于 Deref trait，&mut String 可以自动转换为 &mut str
    s_mut.make_ascii_uppercase();  // 调用 &mut str 的方法
    s_mut.push('!');               // 调用 &mut String 的方法
}
```

## 8. 关键要点总结

1. **存储位置**：

   - 字符串字面值存储在程序二进制文件的只读数据段
   - `&str` 变量本身存储在栈上
   - `String` 数据存储在堆上

2. **可变性区分**：

   - 变量绑定的可变性 (`let mut`)
   - 数据内容的可变性 (`&` vs `&mut`)

3. **类型层次**：

   - `&str` ← 字符串切片的不可变引用
   - `&mut str` ← 字符串切片的可变引用（不能改长度）
   - `&String` ← String 的不可变引用
   - `&mut String` ← String 的可变引用（可以改长度）

4. **使用建议**：
   - 函数参数优先使用 `&str`（更通用）
   - 需要修改内容时使用 `&mut String`
   - `&mut str` 使用场景相对较少

## 9. 在 Solana 开发中的应用

这些概念在 Solana 合约开发中非常重要：

```rust
// 处理账户数据
pub fn process_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 字符串处理和数据验证
    let accounts: Vec<_> = accounts
        .iter()
        .filter(|acc| acc.is_signer)
        .collect();

    // 错误处理
    let account = accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;

    Ok(())
}
```

掌握这些字符串类型的区别对于编写安全、高效的 Rust 代码至关重要。
