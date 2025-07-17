// Solana转账可能的结果
#[derive(Debug)]
enum TransferResult {
    Success,        // 转账成功
    InsufficientBalance,  // 余额不足
    AccountNotFound,      // 账户不存在
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

}


fn print_transfer_result(result: TransferResult) {
    match result {
        TransferResult::Success => println!("转账成功"),
        TransferResult::InsufficientBalance => println!("余额不足"),
        TransferResult::AccountNotFound => println!("账户不存在"),
    }
}