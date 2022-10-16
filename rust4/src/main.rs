// 学习rust的所有权

fn main1() {
    // let s  = "string";
    let mut s1 = String::from("hello");
    s1.push_str(", world");
    println!("s1 =  {}", s1);
    {
        // let s = String::from("hello");
        // 执行与s相关的操作
    } // 作用域到这里结束,变量s失效,这会调用一个drop函数
    let s2 = s1; // 此时s1会失效
                 //s1.push_str("now");
    println!("s2 =  {}", s2);
    let ownership = String::from("hello");
    takes_ownership(ownership);
    // println!("ownership = {}",ownership);
    // 从这里开始ownership不再有效
    let x = 4;
    makes_copy(x);
    // x依然可用
    println!("x = {}", x);
}

fn takes_ownership(some_string: String) {
    println!("some_string {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
}

fn main3() {
    let s1 = gives_ownership(); // gives_ownership 将返回值移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到 takes_and_gives_back 中,它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域

    some_string // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}

fn main2() {
    let s1 = String::from("hello");
    // let z = calculate_length_2(&s1);
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is '{}' ", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
fn calculate_length_2(s: &String) -> usize {
    //使用String的引用作为参数而没有直接转移值的所有权
    s.len()
}

fn main() {
    let mut s = String::from("hello");
    s.push_str("1");
    change_2(&mut s);
    s.push_str("2");
    let s1 = &mut s;
    println!("s1 = {}", s1);
}
/*fn change(some_string: &String) {
    some_string.push_str(",world");
}*/
fn change_2(some_string: &mut String) {
    some_string.push_str(",world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //转为字节数组
    for (i, &item) in bytes.iter().enumerate() {
        //iter方法创建一个可以遍历字节数组的迭代器,enumerate()函数返回一个元组
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_world_3(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
