fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "你好!";
    let english = "Hello!";

    let regions = [southern_germany, chinese, english];

    for region in regions.iter() { //iter() 返回&(&str)
        println!("{}", &region);
    }

    println!("{}", regions[0]);
}

fn ownership_test() {
    // 这个例子会演示所有权转移的问题
    let strings = [
        String::from("Hello"),
        String::from("World"),
        String::from("Rust"),
    ];

    // 这会移动所有权，导致后面无法使用strings
    for s in strings {
        println!("{}", s);
    }

    // 这行会编译错误，因为strings的所有权已经被移动了
    // println!("{}", strings[0]);
}

fn ownership_solution() {
    let strings = [
        String::from("Hello"),
        String::from("World"), 
        String::from("Rust"),
    ];

    // 解决方案1：使用引用迭代
    for s in &strings {
        println!("{}", s);
    }

    // 现在可以正常访问
    println!("First: {}", strings[0]);

    // 解决方案2：使用iter()
    for s in strings.iter() {
        println!("{}", s);
    }
}

fn type_examples() {
    println!("=== 类型识别示例 ===");
    
    // 1. 字符串字面量是 &str（引用类型，实现了Copy）
    let text1 = "Hello";  // 类型: &str
    let text2 = "World";  // 类型: &str
    
    // 2. String::from() 创建的是 String（拥有类型，没有实现Copy）
    let owned1 = String::from("Hello"); // 类型: String
    let owned2 = String::from("World"); // 类型: String
    
    // 3. 数字类型都实现了Copy
    let _number = 42;      // 类型: i32
    let _float = 3.14;     // 类型: f64
    
    // 4. 布尔类型实现了Copy
    let _flag = true;      // 类型: bool
    
    // 让我们看看这些在循环中的行为
    println!("测试 &str 数组:");
    let str_array = [text1, text2, "Rust"];
    for item in str_array {
        println!("  {}", item);
    }
    println!("  str_array 在循环后仍可访问: {}", str_array[0]); // 这会工作
    
    println!("测试 String 数组:");
    let string_array = [owned1, owned2, String::from("Rust")];
    for item in string_array {
        println!("  {}", item);
    }
    // println!("  string_array[0]: {}", string_array[0]); // 这会编译错误
    
    println!("测试数字数组:");
    let number_array = [1, 2, 3];
    for item in number_array {
        println!("  {}", item);
    }
    println!("  number_array 在循环后仍可访问: {}", number_array[0]); // 这会工作
}

fn check_types_with_compiler() {
    println!("\n=== 让编译器告诉我们类型 ===");
    
    let mystery1 = "Hello";
    let mystery2 = String::from("Hello");
    let mystery3 = 42;
    
    // 故意写错误的类型，编译器会告诉我们正确的类型
    // let _: i32 = mystery1;  // 取消注释看编译错误
    // let _: &str = mystery2; // 取消注释看编译错误
    // let _: String = mystery3; // 取消注释看编译错误
    
    println!("mystery1 = {}", mystery1);
    println!("mystery2 = {}", mystery2);
    println!("mystery3 = {}", mystery3);
}

fn copy_trait_examples() {
    println!("\n=== Copy trait 的行为演示 ===");
    
    // 实现了Copy的类型
    let a = 42;
    let b = a;        // 这是复制，不是移动
    println!("a = {}, b = {}", a, b); // a 仍然可用
    
    let x = "hello";
    let y = x;        // 这是复制，不是移动
    println!("x = {}, y = {}", x, y); // x 仍然可用
    
    // 没有实现Copy的类型
    let s1 = String::from("hello");
    let s2 = s1;      // 这是移动，不是复制
    // println!("s1 = {}", s1); // 这会编译错误，因为s1已经被移动
    println!("s2 = {}", s2);
    
    // 如果想要复制String，需要显式clone
    let s3 = String::from("world");
    let s4 = s3.clone(); // 显式复制
    println!("s3 = {}, s4 = {}", s3, s4); // 都可用
}

fn main() {
    // println!("=== greet_world (works because &str implements Copy) ===");
    // greet_world();
    
    // println!("\n=== ownership_test ===");
    // ownership_test();
    
    // println!("\n=== ownership_solution ===");
    // ownership_solution();

    // type_examples();
    // check_types_with_compiler();
    // copy_trait_examples();

    // let region = &"Hello";
    // let deref_once = *region;
    // let deref_twice = &region;
    // let x : &str = deref_twice;
    // println!("region = {}", region);
    // println!("deref_once = {}", deref_once);
    // println!("deref_twice = {}", deref_twice);
    // println!("x = {}", x);


    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";
    
    let records = penguin_data.lines(); // lines() 是懒加载迭代器，这里其实没有分割，只有在遍历时才会逐行处理，<'_>中的'_表示生命周期，让编译器自动推断，和penguin_data的生命周期一致
    
    for (i, record) in records.enumerate() { // enumerate() 返回(index, &str)
        if i == 0 || record.trim().len() == 0 { // trim是&str的方法，返回&str
        continue;
        }
    
        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record
        .split(',')
        .map(|field| field.trim())
        .collect();
        if cfg!(debug_assertions) {
            // 输出到标准错误输出
        eprintln!("debug: {:?} -> {:?}",
                record, fields);
        }
    
        let name = fields[0];
        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            // 输出到标准输出
            println!("{}, {}cm", name, length);
        }
    }

}
