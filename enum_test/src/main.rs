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