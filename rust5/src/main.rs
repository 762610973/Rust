fn main() {
    let s = String::from("hello world");
    let a = &s[..5];
    let b = &s[6..];
    println!("a = {}", a);
    println!("b = {}", b);
    println!("s = {}", s);
}
