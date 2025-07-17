// Solana合约开发中的Trait与泛型基础 - 实践代码

use std::fmt;

// ===============================
// 1. 基础 Trait 定义和实现
// ===============================

// 定义一个Summary trait，类似于Solana中的账户处理trait
pub trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现，类似于Solana中的默认验证逻辑
    fn validate(&self) -> bool {
        !self.summarize().is_empty()
    }
}

// 模拟Solana账户结构
#[derive(Debug, Clone, PartialEq)]
pub struct TokenAccount {
    pub mint: String,     // 在实际Solana中是Pubkey
    pub owner: String,    // 在实际Solana中是Pubkey
    pub amount: u64,
}

// 为TokenAccount实现Summary trait
impl Summary for TokenAccount {
    fn summarize(&self) -> String {
        format!("Token账户: owner={}, mint={}, amount={}", 
                self.owner, self.mint, self.amount)
    }
}

// 另一个账户类型
#[derive(Debug, Clone)]
pub struct UserAccount {
    pub username: String,
    pub balance: u64,
    pub created_at: i64,
}

impl Summary for UserAccount {
    fn summarize(&self) -> String {
        format!("用户账户: {}, 余额: {}", self.username, self.balance)
    }
}

// ===============================
// 2. 特征作为函数参数
// ===============================

// 使用impl Trait语法 - 类似于Solana中的账户验证函数
pub fn process_account(account: &impl Summary) {
    println!("处理账户: {}", account.summarize());
    println!("验证结果: {}", account.validate());
}

// 使用特征约束语法 - 更灵活的写法
pub fn validate_and_process<T: Summary + fmt::Debug>(account: &T) {
    println!("调试信息: {:?}", account);
    println!("账户摘要: {}", account.summarize());
    
    if account.validate() {
        println!("✓ 账户验证通过");
    } else {
        println!("✗ 账户验证失败");
    }
}

// ===============================
// 3. 泛型基础
// ===============================

// 泛型函数 - 类似于Solana中的通用数据处理
fn serialize_data<T: fmt::Debug>(data: T) -> String {
    format!("{:?}", data)
}

// 泛型结构体 - 用于包装不同类型的账户数据
#[derive(Debug)]
pub struct AccountWrapper<T> {
    pub key: String,      // 在实际Solana中是Pubkey
    pub data: T,
    pub owner: String,    // 在实际Solana中是Pubkey
}

impl<T> AccountWrapper<T> {
    pub fn new(key: String, data: T, owner: String) -> Self {
        Self { key, data, owner }
    }
    
    pub fn get_key(&self) -> &String {
        &self.key
    }
    
    pub fn get_data(&self) -> &T {
        &self.data
    }
}

// 为泛型结构体实现trait
impl<T: Summary> Summary for AccountWrapper<T> {
    fn summarize(&self) -> String {
        format!("包装账户 [{}]: {}", self.key, self.data.summarize())
    }
}

// ===============================
// 4. 模拟Solana合约逻辑
// ===============================

// 模拟CPI调用的结果
#[derive(Debug, PartialEq)]
pub enum TransactionResult {
    Success,
    InsufficientFunds,
    InvalidAccount,
}

// 通用的转账函数 - 类似于Solana中的CPI调用
pub fn transfer_tokens<T: Summary + fmt::Debug>(
    from: &mut T,
    to: &mut T,
    amount: u64,
) -> TransactionResult {
    println!("开始转账:");
    println!("  从: {}", from.summarize());
    println!("  到: {}", to.summarize());
    println!("  金额: {}", amount);
    
    // 模拟转账逻辑
    TransactionResult::Success
}

// ===============================
// 5. 复杂示例：模拟Solana程序
// ===============================

// 模拟程序指令
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
        accounts: Vec<&T>,
    ) -> TransactionResult {
        match instruction {
            ProgramInstruction::Initialize { initial_supply } => {
                println!("初始化程序，初始供应量: {}", initial_supply);
                for account in accounts {
                    println!("  处理账户: {}", account.summarize());
                }
                TransactionResult::Success
            },
            ProgramInstruction::Transfer { amount } => {
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

// ===============================
// 6. 主函数 - 演示所有概念
// ===============================

fn main() {
    println!("=== Solana合约开发中的Trait与泛型基础 ===\n");
    
    // 1. 基础trait使用
    println!("1. 基础Trait使用:");
    let token_account = TokenAccount {
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        owner: "3LKJFWgogznfBhWUk6QqKi9ePeAg6x7J4XR9fFTGw2vG".to_string(),
        amount: 1000,
    };
    
    let user_account = UserAccount {
        username: "alice".to_string(),
        balance: 5000,
        created_at: 1640995200,
    };
    
    process_account(&token_account);
    process_account(&user_account);
    println!();
    
    // 2. 多重特征约束
    println!("2. 多重特征约束:");
    validate_and_process(&token_account);
    validate_and_process(&user_account);
    println!();
    
    // 3. 泛型函数
    println!("3. 泛型函数:");
    let serialized_token = serialize_data(&token_account);
    let serialized_user = serialize_data(&user_account);
    println!("序列化Token账户: {}", serialized_token);
    println!("序列化User账户: {}", serialized_user);
    println!();
    
    // 4. 泛型结构体
    println!("4. 泛型结构体:");
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
    
    println!("包装的Token账户: {}", wrapped_token.summarize());
    println!("包装的User账户: {}", wrapped_user.summarize());
    println!();
    
    // 5. 模拟转账
    println!("5. 模拟转账:");
    let mut from_account = token_account.clone();
    let mut to_account = TokenAccount {
        mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
        owner: "7xKJ2nGnWWvR9mHsq4g8X3T2vE6UyB1RfGfVwYnPt9QE".to_string(),
        amount: 500,
    };
    
    let result = transfer_tokens(&mut from_account, &mut to_account, 100);
    println!("转账结果: {:?}", result);
    println!();
    
    // 6. 程序指令处理
    println!("6. 程序指令处理:");
    let initialize_instruction = ProgramInstruction::Initialize { initial_supply: 1000000 };
    let transfer_instruction = ProgramInstruction::Transfer { amount: 100 };
    
    // 由于不同类型无法放在同一个Vec中，我们分别处理
    let token_accounts = vec![&token_account];
    let user_accounts = vec![&user_account];
    
    let result1 = ProgramProcessor::process_instruction(initialize_instruction, token_accounts);
    let result2 = ProgramProcessor::process_instruction(transfer_instruction, user_accounts);
    
    println!("初始化结果: {:?}", result1);
    println!("转账结果: {:?}", result2);
    println!();
    
    // 7. 展示泛型的威力
    println!("7. 泛型的威力 - 同一个函数处理不同类型:");
    let point_i32 = Point::new(5, 10);
    let point_f64 = Point::new(1.5, 2.5);
    let point_string = Point::new("hello".to_string(), "world".to_string());
    
    println!("整数点: {:?}", point_i32);
    println!("浮点数点: {:?}", point_f64);
    println!("字符串点: {:?}", point_string);
    println!();
    
    println!("=== 学习完成！你现在已经掌握了Trait和泛型的基础知识 ===");
    println!("这些概念在Solana合约开发中无处不在，继续深入学习吧！");
}

// ===============================
// 7. 额外示例：泛型Point结构体
// ===============================

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 为特定类型实现特殊方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// ===============================
// 8. 测试模块
// ===============================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_trait_implementation() {
        let token = TokenAccount {
            mint: "test_mint".to_string(),
            owner: "test_owner".to_string(),
            amount: 100,
        };
        
        assert!(token.validate());
        assert!(token.summarize().contains("Token账户"));
    }
    
    #[test]
    fn test_generic_wrapper() {
        let user = UserAccount {
            username: "test_user".to_string(),
            balance: 1000,
            created_at: 1640995200,
        };
        
        let wrapped = AccountWrapper::new(
            "test_key".to_string(),
            user,
            "test_owner".to_string(),
        );
        
        assert_eq!(wrapped.get_key(), "test_key");
        assert!(wrapped.summarize().contains("包装账户"));
    }
    
    #[test]
    fn test_program_processor() {
        let token = TokenAccount {
            mint: "test_mint".to_string(),
            owner: "test_owner".to_string(),
            amount: 100,
        };
        
        let instruction = ProgramInstruction::Initialize { initial_supply: 1000 };
        let result = ProgramProcessor::process_instruction(instruction, vec![&token]);
        
        assert_eq!(result, TransactionResult::Success);
    }
}