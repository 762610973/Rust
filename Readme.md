# RUST学习🎉️

> rust_strip工具快捷键:Ctrl+Alt+'

- `rustc --version / rustup doc / cargo doc --open`
- Rust有一个静态强类型系统，同时也有类型推断
- 程序文件后缀名：rs
- 命名规范：采用下划线的形式
- 编译：rustc xxx.rs(适合小文件)
- Rust的缩进是4个空格而不是table
- 带`!`的表示该函数是一个宏，不是普通函数
- `Cargo`是Rust的构建系统和包管理器:构建代码、下载依赖库（代码需要的库称作依赖）、编译这些库
- `cargo version`
- `cargo new fileName`
- `cargo init`:初始化一个项目
- `Cargo.toml`:Rust配置文件的格式
- `Cargo.lock`文件确保构建是可重现的，有一个机制确保任何人在任何时候重新构建代码，都会产生相同的结果，除非显示地升级
- `Cargo update`用来更新`crate`到一个新版本
- Rust的代码包称为`crate`
- 源代码都应该放在src目录下
- `cargo build --release`//构建可执行文件
- `cargo run`  //构建和运行文件
- `cargo check`//检查代码，确保能通过编译，但是不产生任何可执行文件，速度快

## rust2:猜数字游戏

- 尽量不要声明未使用的变量，向Go靠拢
- 标准库也被称为std
- `let num1 = 5;` //该变量不可变
- `let mut num2 = 5;`//该变量可变
- `let mut guess_number: String = String::new();`
- `String是一个标准库提供的字符串类型，是UTF-8编码的可增长文本块:String::new()`
- `::`语法表名new是String类型的一个关联函数。关联函数是实现一种特定类型的函数
- `stdin`函数返回有一个`std::io::stdin`的实例，代表终端标准输入句柄的类型
- `read_line`返回`io::Result`类型。`Result`类型是枚举(enumerations)
  ,通常也写作enum,枚举往往与条件表达式match一起使用。`Result`的成员是`Ok和Err`
- `0.8.3`是`^0.8.3`的简写，表示任何至少包含`0.8.3`但低于`0.9.0`的版本
- `use rand::Rng;` `Rng`是一个trait，定义了随机数生成器应实现的方法，想使用这些方法的话，此trait必须在作用域中。
- 区间表达式:`start..end`，包含start，不包含end(1..=100或1..101)
- rust允许用一个新值来遮蔽(shadow)guess之前的值
- `String`实例的`trim`方法会去除字符串开头和结尾的空白字符
- 字符串的`parse`方法将字符串解析成数字
- u32:无符号整数类型 | i32:有符号整数类型

## rust3:通用编程概念

- 常量使用关键字const关键字而不是let关键字声明，并且值的类型必须注明
- rust常量的命名约定是全部字母都是用大写，并且使用下划线分割单词
- 使用let关键字可以遮蔽变量,重复使用let关键字会创建出新的变量,可以在复用变量名称的同时改变它的类型
- rust是一种静态类型的语言
- **标量类型**表示单个值:整型、浮点型、布尔型和字符
	- rust会检查整型溢出
	- rust的字符串类型大小为4个字节，表示的是一个Unicode标量值
- **复合类型**
	- 元组
		- 长度固定，变量绑定到整个元组，因为元组被认作是单个符合元素，想从元组中获取个别值，可以使用模式匹配来结构，也可以通过.+索引的形式
		- `let tup = (1,2.2,"3");`
	- 数组:
		- 希望将数据分配到栈而不是堆时或者明确元素数量不需要改变时，使用数组
- fn定义函数,并且函数和变量名使用下划线命名法(蛇形命名法);函数体由一系列语句组成,也可选择以表达式结尾

> - <span style="color:red;font-size:18px">Rust是一门基于表达式的语言</span>
> - <span style="color:red">表达式计算并产生一个值</span> 大括号是一个表达式
> - <span style="color:red">语句:执行一些操作但不返回值,表达式加上分号就是语句</span> 函数定义是一个语句

- rust中,函数的返回值等同于函数体最后一个表达式的值
- 表达式是返回值的东西。如果输入分号，则会抑制该表达式的结果
- 块的最后一条语句末尾没有分号有一个隐含的含义:值是从块返回的，值很重要。
- 如果该值未被使用，则不要返回它。用分号来帮助人类阅读代码。
- ```rust
  fn return_num(x: i32) -> i32 {
    // return x;  这是一个语句,执行了return操作
    // x+1  这是一个表达式,x+1之后返回了一个值,尽管没有变量去接收,也可以直接返回
    x   // 这是一个表达式,表达式返回的是一个值
  }
  ```
- /* */多行注释 | // 单行注释
- `let condition = true; let number = if condition {5} else {6};`
- 使用loop执行循环程序
- 使用标签`xxx 可以控制外层循环
- 可以从loop中取值,使用break关键字
- while用来条件循环
- `for _val in a {}` 类似python的for,很好用
- 关于分号和语句的问题
- ```rust
  fn main(){
      let number = 4;
      if number == 4 {
          //  
      };
      // 这里的单if语句最后的的分号可以加也可以不加
  }
  ```

## 所有权:为了管理堆数据

> - Rust中每一个值都有一个对应的变量作为它的所有者
> - 在同一时间内,值有且仅有一个所有者
> - 当所有者离开自己的作用域时,它持有的值就会被释放掉
---------------------------------------------

- 变量会保持自己的有效性直到自己离开作用域位置
- 内存分配方式不同导致String是可变的,字符串字面量不是
- 字符串字面量不可变:编译期间就确定了其大小、内容,这部分硬编码的文本被直接嵌入到了最终的可执行文件中,访问高效
- 对于String类型,为了支持一个可变的、可增长的文本类型,需要在堆上分配一块在编译时未知大小的内存来存放数据:
	- 使用的内存是操作系统在运行时动态分配出来的（程序员发起对内存的分配请求）
	- 当时用完String时,需要某种方式来将这些内存归还给操作系统（手动、GC、所有权）
- Rust:内存会自动地在拥有它的变量离开作用域后进行释放
- Rust会在作用域结束的地方自动调用`drop`函数
- Rust不会在复制值时深度地复制堆上的数据
- Rust用于不会自动地创建数据的深度拷贝.在Rust中,任何自动的赋值操作都可以被视为高效的.变量和数据交互的方式:克隆
- 拥有Copy这种trait的类型:所有的整数类型、仅拥有两种值得布尔类型、字符类型、所有的浮点类型,如果元素包含的所有字段的类型都是Copy的,name这个元组也是Copy的
- 变量所有权的转移总是遵循相同的模式:将一个值复制给另一个变量时就会转移所有权.当一个持有堆数据的变量离开作用域时,它的数据就会被drop清理回收,除非这些数据的所有权移动到了变量上

### 引用和借用

- 引用:允许在不获取所有权的前提下使用值(参考代码中的calculate_length_2函数)
- 引用:在离开自己的作用域时销毁其指向的数据,因为它并不拥有该数据的所有权.当一个函数使用引用而不是值本身作为参数时,不需要为了归还所有权而特意去返回值,毕竟在这种情况下,根本没有取得所有权.这种通过引用传递参数给函数的方法被称为借用
- 引用默认是不可变的,Rust不允许去修改指向引用的值
- 借用:通过引用传递参数给函数的方法被称为借用（borrowing）
- 可变引用:对于特性作用域中的特定数据来说,一次只能声明一个可变引用,这条规则可以帮助在编译时避免数据竞争.数据竞争会导致未定义的行为,并且在运行时难以跟踪.,rust中存在数据竞争的代码连编译检查都无法通过
- 同时存在多个不可变引用是合理合法的,对数据的操作不会影响到其他读取数据的用于
- ```rust
  fn main(){
      let mut s = Stirng::from("rust");
      let r1 = &s;  //没问题
      let r2 = &s;  //没问题
      let r3 = &mut s;  //错误,不能在拥有不可变引用的同时创建可变引用.
      let r4 = &mut s;  // 错误,也不能存在两个可变引用
  }
  ```
- ```rust
  fn main(){
      let mut s = Stirng::from("rust");
      let r1 = &s;  //没问题
      let r2 = &s;  //没问题
  	  println!("r1 =  {}",r1);
  	  println!("r2 =  {}",r2);
      let r3 = &mut s;  // 没问题:r1和r2的作用域已经结束了,作用域没有重叠,编译器在作用域结束之前判断不再使用的引用的能力被称为非词法作用域生命周期 
  }
  ```

### 悬垂引用

> 指针指向曾经存在的某处内存地址,但该内存已经被释放掉甚至是被重新分配另作他用了

- Rust编译器会确保引用永远不会进入悬垂状态
-

```rust
fn main() {
    let _reference_to_nothing = dangle();
}

fn dangle() -> &String {            //dangle会返回一个指向String的引用
    let s = String::from("hello");  //s被绑定到新的String上
    &s                              //将指向s的引用返回给调用者
}                                   //变量s在这里离开作用域并随之被销毁,指向的内存自然也不会有效
// 解决办法是直接返回String,所有权被移出函数
```

### 切片:不持有所有权

- 字符串切片:
  指向String对象中某个连续部分的引用`let s = String::from("hello world"); let hello = &s[0..5]; let world = &s[6..1];`
- 切片索引通过[start..end],并且有语法糖

## 使用结构体组织关联数据
- 

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("762610973@qq.com"),
        username: String::from("umbrella"),
        active: true,
        sign_in_count: 1,
    };
    let User2 = User {
        active: user1.active,
        /*username: user1.username,
        email: String::from("new email"),
        sign_in_count: user1.sign_in_count,*/
        ..user1
    };
    // 此时user1的email和username字段不能再使用了,另外两个可以
}
```

- 元组结构体:`struct Color(i32, i32, i32)`
- 类单元结构体:类似Go的空结构,,,,,,,体:`struct Always`,实现trait,而不需要数据
- 结构体没有Display实现
-

```rust
#[derive(Debug)]    //{:?} debug打印,{:#?} 美观地打印
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //实现方法,&self <=> self:&Self
    fn area(&self, a: u32) -> u32 {
        self.width * self.height * a
    }
    fn can_hole(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// 调用时直接用就行了,rust会自动引用和解引用
```

- 所有在impl块中定义的函数被称为关联函数

## 枚举和模式匹配

- 枚举允许通过列举可能的成员来定义一个类型,枚举值只可能是其中一个成员
- 枚举的成员位于其标识符的命名空间中,使用两个冒号分开
- 枚举是一个单独的类型,可以使用impl为枚举定义方法
- Option:一个值要么有值要么没值,如果使用None而不是Some,需要告诉rustOption<T>是什么类型的

```rust
// T是泛型参数,意思是可以是任何类型的数据 
enum Option<T> {
    Some(T),
    None,
}
```

- match允许将一个值与一系列的模式想比较,并根据相匹配的模式执行相关代码.模式可由字面量、变量、通配符和许多其他内容组成
- 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值
- Rust中的匹配时穷举式的:必须穷举到最后的可能性来时代码有效.
- if let语法来处理只匹配一个模式的值而忽略其他模式的情况,是match的一个语法糖,当值匹配某一个模式时执行代码而忽略其它所有值

## 07|使用包、Crate和模块管理不断增长的项目

- 模块系统:
	- 包(Packages):Cargo的一个功能,它允许构建、测试和分享crate
	- Crates:一个模块的属性结构,它形成了库或二进制项目
	- 模块和use(Modules):允许控制作用域和路径的私有性
	- 路径(path):一个命名例如结构体、函数或模块等项的方式

### 包和crate

- crate是一个二进制项或库.crate root是一个源文件,Rust编译器以它为起点,构成crate的跟模块
- 包(package)是提供一系列功能的一个或多个crate.一个包会包含有一个Cargo.toml文件,阐述如何构建这些crate
- 包中内容的规则:
	- 一个包至多只能包含一个库crate
	- 包中可以包含任意多个二进制crate
	- 包中至少包含一个crate
- `src/main.rs`是一个与包同名的二进制crate的crate根.

### 定义模块来控制作用域与私有性

- 模块可以将一个crate中的代码进行分组,以提高可读性与重用性.
- 模块可以控制项的私有性
- 关键字`mod`定义模块,指定模块的名字,模块中可以包含其他模块,也可以包含项、结构体、枚举、常量、trait
- crate
  └── front_of_house
  ├── hosting
  │ ├── add_to_wait_list
  │ └── seat_at_table
  └── serving
  ├── take_order
  ├── serve_order
  └── take_payment

### 路径用于引用模块树中的项

- 路径
	- 绝对路径:从crate根部开始,以crate名或者字面量crate开头
	- 相对路径:从当前模块开始,以self、super或当前模块的标识符开头
- 更倾向于使用绝对路径,因为把代码定义和项目调用各自独立地移动是更常见的
- Rust的私有性边界:不允许外部代码了解、调用和依赖被封装的实现细节
- 默认所有项(函数、方法、结构体、枚举、模块和常量)都是私有的
- **使用pub关键字暴露路径**
- **使用super起始的相对路径**
- 枚举成员默认是公有的(私有化意义不大)

### 使用use关键字将名称引入作用域

- 在作用域中增加use和路径类似于在文件系统中创建软链接
- 使用use引入结构体、枚举和其他项时,习惯是指定他们的完整路径
- 使用父模块将两个具有相同名称的类型引入同一作用域
- 使用use关键字提供新的名称
- 使用`pub use`重导出名称
- 嵌套路径来消除大量的use行:`use std::{cmp::Ordering,io};`
- 通过glob运算符将所有的共有定义引入作用域:`use std::collections::*`
### 将模块分割进不同文件
- 模块定义时,如果模块名后边是";",而不是代码块"{}":
  - Rust会从与模块同名的文件中加载内容
  - 模块树的结构不会变化