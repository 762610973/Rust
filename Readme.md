# RUST学习🎉️
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
- `read_line`返回`io::Result`类型。`Result`类型是枚举(enumerations),通常也写作enum,枚举往往与条件表达式match一起使用。`Result`的成员是`Ok和Err`
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
-  /* */多行注释  |   // 单行注释
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