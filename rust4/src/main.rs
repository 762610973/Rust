// 学习rust的所有权
fn main() {
    println!("学习所有权");
    let number = 5;
    for _element in 1..5 {
        println!("{}",_element);
        println!("number is {}",number);
    }
    println!("end");
    let (a,b) = test();
    println!("a =  {}",a);
    println!("b =  {}",b);

}

fn test() -> (i32,f32) {
    return (3,3.3);
}