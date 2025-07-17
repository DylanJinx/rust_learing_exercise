fn main() {
    let x = 5;
    // 在main函数的作用域内对之前的x进行遮蔽
    let x = x + 1;

    {
        // 在当前的花括号作用域内，对之前的x进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    let x = 60;

    println!("The value of x is: {}", x);

    let guess: i32 = "42".parse().expect("Not a number!");

    let mut s = "hello";
    println!("s 指向: {}, 地址: {:p}", s, s.as_ptr());

    s = "world";
    println!("s 指向: {}, 地址: {:p}", s, s.as_ptr());

    let mut s = String::from("hello");

    let s_mut = &mut s;
    s_mut.push('a');
    println!("{}", s_mut);

    let s_mut_str: &mut str = &mut s;
    s_mut_str.make_ascii_uppercase();
    // s_mut_str.push('x');
    println!("{}", s_mut_str);
}
