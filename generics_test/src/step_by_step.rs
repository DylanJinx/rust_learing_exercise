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

// ===============================
// 4. 模拟Solana合约逻辑
// ===============================
#[derive(Debug, PartialEq)]
pub enum TransactionResult {
    Success,
    InsufficientFunds,
    InvalidAccount,
}

pub fn transfer_tokens<T: Summary + fmt::Debug, U: Summary + fmt::Debug>(
    from: &T,
    to: &U,
    amount: u64,
) -> TransactionResult {
    println!("开始转账：");
    println!("从：{}", from.summarize());
    println!("  到: {}", to.summarize());
    println!("  金额: {}", amount);

    if amount == 0 {
        TransactionResult::InvalidAccount
    } else if amount > 10000 {
        TransactionResult::InsufficientFunds
    } else {
        TransactionResult::Success
    }
}

// 处理交易结果
pub fn handle_transaction_result(result: TransactionResult) {
    match result {
        TransactionResult::Success => {
            println!("✅ 交易成功!");
        },
        TransactionResult::InsufficientFunds => {
            println!("❌ 余额不足!");
        },
        TransactionResult::InvalidAccount => {
            println!("❌ 账户无效!");
        },
    }
}

#[derive(Debug)]
pub enum ProgramInstruction {
    Initialize { initial_supply: u64 },
    Transfer { amount: u64 },
    Mint { amount: u64 },
}

// 程序处理器 - 使用泛型处理不同类型的账户
pub struct ProgramProcessor;

impl ProgramProcessor {
    pub fn process_instruction<T: Summary + fmt::Debug>(
        instruction: ProgramInstruction,
        account: &T,
    ) -> TransactionResult {
        match instruction {
            ProgramInstruction::Initialize {
                initial_supply
            } => {
                println!("初始化程序，初始供应量: {}",   initial_supply);
                    println!("  处理账户: {}", account.summarize());
                TransactionResult::Success
            },
            ProgramInstruction::Transfer {
                amount
            } => {
                println!("执行转账，金额: {}", amount);
                TransactionResult::Success
            },
            ProgramInstruction::Mint { amount } => {
                println!("铸造代币，数量: {}", amount);
                TransactionResult::Success
            },
        }
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

    // 新增：测试转账和结果处理
    println!("\n--- 测试转账和结果处理 ---");

    // 测试成功转账
    let result1 = transfer_tokens(&token_account, &user_account, 100);
    handle_transaction_result(result1);

    // 新增：测试程序指令处理
    println!("\n--- 测试程序指令处理 ---");
    // 测试初始化指令

    let initialize_instruction = ProgramInstruction::Initialize { initial_supply: 1000000 };
    let account = &token_account;
    let result = ProgramProcessor::process_instruction(initialize_instruction, account);
    handle_transaction_result(result);

    let transfer_instruction = ProgramInstruction::Transfer { amount: 500 };
    let user_account = &user_account;
    let result = ProgramProcessor::process_instruction(transfer_instruction, user_account);
    handle_transaction_result(result);

    let mint_instruction = ProgramInstruction::Mint { amount: 1000 };
    let wrapped_account = &wrapped_token;
    let result = ProgramProcessor::process_instruction(mint_instruction, wrapped_account);
    handle_transaction_result(result);



}
