fn main() {
    println!("Hello, world!");
    let mut condition = 5;   // 这是一个语句,语句执行一些指令
    let number = {
        condition += 1;
        condition * 4
    };
    println!("number =  {}",number);
    if number == 24 {
        println!("关于println的分号问题,有时候可以加,有时候可以不加")
        // 这个是最后一行的时候就可以不加
    }
}
