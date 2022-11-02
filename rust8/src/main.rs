#![allow(unused)]

mod str1;
// use str1 as ttt;
fn main() {
    // ttt::test_string();
    let v1: Vec<i32> = Vec::new();
    // 通过宏创建,自动推断类型
    let v2 = vec![1, 2, 3];
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("v = {:?}", v);
    v.pop();
    v.pop();
    v.pop();
    println!("v = {:?}", v);
    read_val_from_vector();
    read_val_from_vector_but_none_val();
    test_rule();
    range_vector();
    range_vector_add();
    test_enum();
    str1::test_string();
    str1::test_string_index();
    str1::test_map();
}
fn read_val_from_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
}
fn read_val_from_vector_but_none_val() {
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    let does_not_exist_2 = v.get(2);
    match does_not_exist_2 {
        None => println!("does not have this element"),
        Some(does) => println!("does = {}", does),
    }
}
fn test_rule() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = v[0];
    v.push(6);
    println!("first = {}", first);
}
fn range_vector() {
    let v = vec![1, 2, 3];
    for i in &v {
        println!("i = {}", i);
    }
}
fn range_vector_add() {
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        // 修改可变引用所指向的值,在使用+=运算符之前必须使用解引用运算符(*)获取i中的值
        *i += 10;
    }
}
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Test(String),
}
fn test_enum() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Test(String::from("妙呀")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row = {:?}", row);
}
