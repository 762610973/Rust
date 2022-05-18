use rand::Rng;
use std::io;    // prelude 显式导入
use std::cmp::Ordering;


fn main(){
    println!("start");
    println!("猜数"); //带着 ！是一个宏
    let secret_number = rand::thread_rng().gen_range(1,101); //i32,u32,i64
    println!("神秘数字是：{}",secret_number);
    loop {
        println!("猜测一个数");
        // let foo = 1;
        //let bar = foo;// immutable 不可变
        let mut guess: String = String :: new();//声明一个可修改的变量
        io::stdin().read_line(&mut guess).expect("无法读取行");
        //                         &表示是一个引用，默认不可变
        // io :: Result Ok,Err,
        println!("你猜测的数是：{}",guess); //{}一个占位符

        //let guess: u32 = guess.trim().parse().expect("转换失败，请输入一个数字！");
        let guess:u32 = match guess.trim().parse() {
          Ok(num) => num,
            Err(_) => continue,
        };
        //       显示声明类型


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

}