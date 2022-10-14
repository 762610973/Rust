const THE_CONST_NUMBER: i32 = 100;
fn main() {
    test_for_2();
    test_for();
    return_loop();
    double_loop();
    let mut x = 5;
    println!("{}", x);
    x = 6;
    println!("{}", x);
    println!("{}", THE_CONST_NUMBER);
    {
        // 内部作用域
        let x = x * 2;
        println!("{}", x);
    }
    println!("{}", x);
    let s = 3;
    println!("{}", s);
    let s = "qwer";
    println!("{}", s);
    let f1 = 2.0;
    let f2: f32 = 3.0;
    let t1 = true;
    let t2 = false;
    println!("{}{}{}{}", f1, f2, t1, t2);
    let c1 = 'z';
    let c2 = "Z";
    println!("{} -- {}", c1, c2);
    let mut t1 = (500, 6.4, 1);
    t1.0 = 100;
    let (x1, y1, z1) = t1;
    println!("The value is:{} {} {}", x1, y1, z1);
    println!("{}", x1);
    println!("{}", t1.1);
    let t2 = (1, 2, 3);
    println!("{}", t2.0);
    // let array1 = [1, 2, 3];

    // let array2: [i32; 3] = [1, 2, 3];
    let mut array3 = [3; 5]; //一共五个元素,默认值是3
    println!("{}", array3[3]);
    array3[3] = 10;
    println!("{}", array3[3]);
    another_function();
    let num = five(3);
    println!(" {}", num);
    six();
    let number = 9;
    if number % 3 == 0 {
        println!("9 整除 3")
    }
    if number == 8 {
        println!("hello")
    } else if number == 10 {
        println!("rust")
    } else {
        println!("hello rust")
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
    println!(" {}", number);
    println!("$ $ {}", number);
    println!(" {}", number);
}

fn another_function() {
    println!("Another function.");
}

fn five(mut x: i32) -> i32 {
    let y = {
        x *= 3;
        x + 1 // 这里是一个语句,并没有对x产生影响,因此外面的x是9
    };
    {
        let x = -3;
        println!("括号里面的x是 {}", x);
    }
    println!("括号外面的x是 {}", x);
    // return y;    可以使用return 返回
    // 100 可以直接写一个数
    y + 1 // 现在是一个表达式,y+1返回了一个值
}

fn six() {
    let x = "six";
    println!("first x =  {}", x);
    let x = x.len();
    println!("second x =  {}", x);
    let x = "six";
    println!("first x =  {}", x);
    let mut y = 100;
    y += 1;
    println!("first y = {}", y);
    let mut y = "123";
    println!(" {}", y);
    y = "1234";
    println!("second y =  {}", y);
    let y = 123;
    println!(" {}", y);
    /*
        多行注释
    */
    // 单行注释
}

fn double_loop() {
    // 官方给的例子非常不错
    let mut count = 0;
    'containing_up: loop {
        println!("count =  {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 7 {
                break;
            }
            if count == 3 {
                break 'containing_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count =  {}", count);
}

fn return_loop() {
    let mut counter = 0;
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("res =  {}", res);
}
fn test_for() {
    let mut number = 3;
    while number != 0 {
        println!("number =  {}", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for _val in a {
        println!("the value is: {}", _val);
    }
    println!("a.len() =  {}", a.len());
}

fn test_for_2() {
    for _number in (1..4).rev() {
        println!("number: {}", _number)
    }
    println!("LIFTOFF!!!");
    let a = [11, 12, 13, 14, 15];
    for _element in a.iter() {
        println!("element =  {}", _element);
    }
}
