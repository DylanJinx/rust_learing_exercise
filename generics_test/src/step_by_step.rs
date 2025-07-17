use std::fmt;

pub trait Summary {
    fn summarize(&self) -> String;

    fn validate(&self) -> bool {
        !self.summarize().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenAccount {
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

impl Summary for TokenAccount {
    fn summarize(&self) -> String {
        format!("Token account: owner = {}, mint = {}, amount = {}", self.owner, self.mint, self.amount)
    }
}

#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub balance: u64,
    pub create_at: i64,
}

impl Summary for UserAccount {
    fn summarize(&self) -> String {
        format!("user account: username = {}, balance = {}, create_at = {}", self.username, self.balance, self.create_at)
    }
}

// ===============================
// 2. 特征作为函数参数
// ===============================
pub fn process_account(account: &impl Summary) { // impl Summary：任何实现了Summary trait的类型
    println!("处理账户: {}", account.summarize());
    println!("验证结果: {}", account.validate());
}

// T: Summary + fmt::Debug：
//   - 这意味着类型 T 必须同时实现 Summary 和 fmt::Debug 两个 trait
//   - 只有满足这两个条件的类型才能作为参数传入
pub fn validate_and_process<T: Summary + fmt::Debug>(account: &T) {
    println!("调试信息： {:?}", account);
    println!("账户摘要： {}", account.summarize());

    if account.validate() {
        println!("✓ 账户验证通过");
    } else {
        println!("✗ 账户验证失败");
    }
}


// ===============================
// 3. 泛型基础
// ===============================
// T: fmt::Debug表示T必须实现Debug trait
fn serialize_data<T: fmt::Debug>(data: T) -> String {
    format!("{:?}", data)
}

fn get_summary<T: Summary>(item: &T) -> String {
    item.summarize()
}

#[derive(Debug)]
pub struct AccountWrapper<T> {
    pub key: String,
    pub data: T,
    pub owner: String,
}

impl<T> AccountWrapper<T> {
    pub fn new(key: String, data: T, owner: String) -> Self {
        Self {key, data, owner}
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }

    pub fn get_data(&self) -> &T {
        &self.data
    }
}

// 只有当T实现了Summary时，AccountWrapper<T>才能实现Summary
impl<T: Summary> Summary for AccountWrapper<T> {
    fn summarize(&self) -> String {
        format!("wrapping account [{}]: data = \"{}\", owner = {}", self.key, self.data.summarize(), self.owner)
    }
}


fn main() {
    let token_account = TokenAccount {
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        owner: "3LKJFWgogznfBhWUk6QqKi9ePeAg6x7J4XR9fFTGw2vG".to_string(),
        amount: 1000,
    };

    println!("摘要： {}", token_account.summarize());
    println!("验证： {}", token_account.validate());

    //使用trait作为函数参数
    println!("\n--- 使用函数处理账户 ---");
    process_account(&token_account);

    let user_account = UserAccount {
        username: "alice".to_string(),
        balance: 5000,
        create_at: 1640995200,
    };

    println!("\n--- 处理用户账户 ---");
    process_account(&user_account);

    // serialize_data 可以处理任何实现了Debug的类型
    let serialized_token = serialize_data(&token_account);
    let serialized_user = serialize_data(&user_account);
    let serialized_number = serialize_data(42);
    let serialized_string = serialize_data("hello");

    println!("序列化token: {}", serialized_token);
    println!("序列化user: {}", serialized_user);
    println!("序列化数字: {}", serialized_number);
    println!("序列化字符串: {}", serialized_string);

    // get_summary 可以处理任何实现了Summary的类型
    println!("\n使用泛型获取摘要:");
    println!("Token摘要: {}", get_summary(&token_account));
    println!("User摘要: {}", get_summary(&user_account));

    // 测试泛型结构体
    println!("\n--- 测试泛型结构体 ---");

    let wrapped_token = AccountWrapper::new(
        "TokenAccount123".to_string(), 
        token_account.clone(), 
        "SystemProgram".to_string(),
    );

    let wrapped_user = AccountWrapper::new(
        "UserAccount456".to_string(),
        user_account.clone(),
        "MyProgram".to_string(),
    );

    println!("包装的Token: {}", wrapped_token.summarize());
    println!("包装的User: {}", wrapped_user.summarize());

    // 同一个函数可以处理包装后的账户
    println!("\n处理包装后的账户:");
    process_account(&wrapped_token);
    process_account(&wrapped_user);

    // 新增：测试多重特征约束
    println!("\n--- 测试多重特征约束 ---");
  
        // 这个函数需要类型同时实现Summary和Debug
        validate_and_process(&token_account);
        println!();
        validate_and_process(&user_account);
        println!();
        validate_and_process(&wrapped_token);
        validate_and_process(&wrapped_user);
}
