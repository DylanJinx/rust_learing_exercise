fn main() {
    let array = [
        String::from("rust is good!"),
        String::from("rust is good!"),
        String::from("rust is good!"),
    ];

    println!("{:#?}", array);

    let array: [String; 8] = std::array::from_fn(|_i| String::from("rust is good!"));
    println!("{:#?}", array);
}
