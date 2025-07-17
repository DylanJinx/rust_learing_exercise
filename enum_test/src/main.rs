// Solana转账可能的结果
#[derive(Debug)]
enum TransferResult {
    Success,        // 转账成功
    InsufficientBalance,  // 余额不足
    AccountNotFound,      // 账户不存在
}

#[derive(Debug)]
enum SolanaInstruction {
    Transfer { amount: u64, to_address: String },
    CreateAccount { initial_balance: u64 },
    CloseAccount,
}

fn main() {
    let a = TransferResult::Success;
    let b = TransferResult::InsufficientBalance;
    let c = TransferResult::AccountNotFound;

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    print_transfer_result(a);
    print_transfer_result(b);
    print_transfer_result(c);

    let solana_instruction_a = SolanaInstruction::Transfer { amount: 100, to_address: String::from("0x1234567890") };
    let solana_instruction_b = SolanaInstruction::CreateAccount { initial_balance: 100 };
    let solana_instruction_c = SolanaInstruction::CloseAccount;

    println!("{:?}", solana_instruction_a);
    println!("{:?}", solana_instruction_b);
    println!("{:?}", solana_instruction_c);

    print_solana_instruction(solana_instruction_a);
    print_solana_instruction(solana_instruction_b);
    print_solana_instruction(solana_instruction_c);

    let account_balance = find_account("0x1234567890");
    let new_balance = match account_balance {
        Some(balance) => {
            balance * 10_u64.pow(9)
        }
        None => {
            0
        }
    };
    println!("{:?}", new_balance);

    let account_balance = find_account("0x1234567891");
    let new_balance = match account_balance {
        Some(balance) => {
            balance * 10_u64.pow(9)
        }
        None => 0,
    };
    println!("{:?}", new_balance);

    let account_balance = find_account("0x1234567892");
    let new_balance = match account_balance {
        Some(balance) => {
            balance * 10_u64.pow(9)
        }
        None => 0,
    };
    println!("{:?}", new_balance);
    
    let account_balance_error = find_account("0x1234567893");
    let new_balance = match account_balance_error {
        Some(balance) => {
            balance * 10_u64.pow(9)
        }
        None => 0,
    };
    println!("{:?}", new_balance);

    let _some_number = Some(5);
    let _some_string = Some("Hello");
    let _some_bool = Some(true);

    let _none_number: Option<i32> = None;

    let new_balance = transfer_sol(1000, 100);
    match new_balance {
        Ok(balance) => {
            println!("{}", balance);
        },
        Err(s) => {
            println!("{}", s)
        }
    };

    let new_balance = complex_transfer("0x1234567890", "0x1234567891", 50);
    println!("{:?}", new_balance);

}

fn print_transfer_result(result: TransferResult) {
    match result {
        TransferResult::Success => println!("转账成功"),
        TransferResult::InsufficientBalance => println!("余额不足"),
        TransferResult::AccountNotFound => println!("账户不存在"),
    }
}

fn print_solana_instruction(instruction: SolanaInstruction) {
    match instruction {
        SolanaInstruction::Transfer { amount, to_address} => {
            println!("转账 {} 到 {}", amount, to_address);
        }
        SolanaInstruction::CreateAccount { initial_balance } => {
            println!("创建账户，初始余额 {}", initial_balance);
        }
        SolanaInstruction::CloseAccount => {
            println!("关闭账户");
        }
    }
}

fn find_account(address: &str) -> Option<u64> {
    match address {
        "0x1234567890" => Some(100),
        "0x1234567891" => Some(200),
        "0x1234567892" => Some(300),
        _ => None,
    }
}

fn transfer_sol(
    from_balance: u64,
    amount: u64
) -> Result<u64, String> { // 成功时返回u64，失败时返回String
    if amount > from_balance {
        Err("余额不足".to_string()) // Err(值): 失败，包含错误信息
    } else {
        Ok(from_balance - amount) // Ok(值): 成功，包含结果
    }
}

fn complex_transfer(
    from: &str,
    to: &str,
    amount: u64
) -> Result<u64, String> {
    let from_balance = find_account(from).ok_or("发送方账户不存在")?;
    let _to_balance = find_account(to).ok_or("接收方账户不存在")?;
    transfer_sol(from_balance, amount)
}