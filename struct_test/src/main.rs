#[derive(Debug)]
struct Wallet {
    _address: String,
    _amount: u64,
}

fn main() {
    let mut wallet = Wallet {
        _address: String::from("ABC123"),
        _amount: 500,
    };
    wallet = dbg!(wallet);
    println!("{:?}", wallet);

    let mut wallet2 = dbg!(wallet);
    // println!("{:?}", wallet);
    println!("{:?}", wallet2);
    wallet2 = dbg!(wallet2);
    println!("{:?}", wallet2);
}
