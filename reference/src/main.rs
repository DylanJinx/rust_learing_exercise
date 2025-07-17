fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 不可变借用开始
    let r2 = &s; // 不可变借用开始
    println!("{} and {}", r1, r2); // r1, r2 最后一次使用
                                   // ← r1, r2 作用域在这里结束（NLL - Non-Lexical Lifetimes）

    let r3 = &mut s;
    println!("{}", r3);

    // println!("{} and {}", r1, r2);
} // 老编译器中，r1、r2、r3作用域在这里结束
  // 新编译器中，r3作用域在这里结束
