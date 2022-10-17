/*fn main1() {
    let s = String::from("hello world");
    let a = &s[..5];
    let b = &s[6..];
    println!("a = {}", a);
    println!("b = {}", b);
    println!("s = {}", s);
    let mut ss = String::from("rust");
    let r1 = &ss; //没问题

    let r2 = &ss; //没问题
    println!("r1 =  {}", r1);
    // ss.push_str("golang");   // 这一行不行,因为有不可变的引用了
    println!("r2 =  {}", r2);
    // ss.push_str("golang");   // 这一行可以
    let r3 = &mut ss;
    println!("r3 = {}", r3);
}*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32); // 元组结构体
struct Always; // 类单元结构体,类似Go的空结构体,实现trait,但是不需要数据
fn main() {
    let black = Color(0, 0, 0);
    let point = Point(0, 0, 0);
    println!("black.0 = {}", black.0);
    println!("point.0 = {}", point.0);
    println!("point.1  {}", point.1);

    let user1 = User {
        email: String::from("762610973@qq.com"),
        username: String::from("umbrella"),
        active: true,
        sign_in_count: 1,
    };
    // user1.username = String::from("jxl_umbrella");

    /*let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("new email"),
        sign_in_count: user1.sign_in_count,
    };*/
    let user2 = User {
        // email: String::from("user2"),
        ..user1
    };
    //user1.active = false;
    println!("user1.active = {}", user1.active);
    /*
    // 这两个都不行,因为email和username都已经被move到了user2中
    println!("user1.username = {}",user1.username);
    println!("user1.active = {}", user1.email);
    */

    println!("user2.sign_in_count = {}", user2.username);
    println!("user2.sign_in_count = {}", user2.email);
    println!("user2.sign_in_count = {}", user2.active);
    println!("user2.sign_in_count = {}", user2.sign_in_count);
    //main1();
}
/*fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 参数名与字段名完全相同,可以简化
fn build_user_2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
*/

fn area(dimensions:(u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width:u32,
    height:u32,
}

fn area_1(rectangle:&Rectangle) -> u32{
    rectangle.height*rectangle.width
}
