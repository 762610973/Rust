use rand::Rng;
use std::cmp::Ordering;
use std::io;
pub fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    //println!("The secret number is: {}", secret_number);

    loop {
        println!("please input your guess.");
        let mut guess_number: String = String::new();
        // 输入数字
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Failed to read Line");
        // 忽略非数字，将expect调用换成match语句，从而实现遇到错误就崩溃转换成处理错误
        let guess_number: i32 = match guess_number.trim().parse() {
            Ok(num) => num,
            // Err(_) => continue,
            Err(_) => continue,
        };
        // let guess_number: i32 = guess_number.trim().parse().expect("Please type a number!");

        println!("Your guessed:{}", guess_number);
        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
