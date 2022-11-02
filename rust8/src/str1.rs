use std::collections::HashMap;
pub fn test_string() {
    let mut s = String::new();
    println!("s = {}", s);
    let data = "initial contents";
    println!("data = {}", data);
    let s = data.to_string();
    println!("s = {}", s);
    let s = "initial contents".to_string();
    println!("s = {}", s);
    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 = {}", s2);
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 = {}", s3);
    let a1 = String::from("1");
    let a2 = String::from("2");
    let a3 = a1 + &a2;
    // 所有权转移到了a3中
    // println!("a1 = {}",a1);
    println!("s2 = {}", s2);
    println!("a3 = {}", a3);
    let b1 = String::from("tic1");
    let b2 = String::from("tic2");
    let b3 = String::from("tic3");
    let b4 = format!("{}-{}-{}-{}", s1, s2, s3, "end");
    println!("b4 = {}", b4);
}

pub(crate) fn test_string_index() {
    let s = String::from("我正在学习Rust");
    let s_len = s.len();
    println!("s_len is {}", s_len);
    for c in s.chars() {
        println!("c = {}", c);
    }
    for b in s.bytes() {
        println!("b = {}", b);
    }
}

pub(crate) fn test_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let s = scores.get("Blue");
    println!("s = {:?}", s);
    println!("Blue {:?}", scores.get("Blue"));
    println!("grey {:?}", scores.get("grey"));
    scores.insert(String::from("Blue"), 10000);
    println!("scores = {:?}", scores);

    let teams = vec![String::from("1"), String::from("2")];
    let initial_scores = vec![10, 20];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    /*for (key, value) in &scores {
        println!("{}->{}", key, value);
    }*/
    /*for (k, v) in scores {
        println!("{}->{}", k, v);
    }*/
    scores.entry(&String::from("red")).or_insert(&1000);
}
