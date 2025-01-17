use std::io;
use std::fmt::Arguments;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;
use ferris_says::say;


// 使用pub将函数定义为公共函数，以便外部访问
pub fn variables_mutability_1() {
    // rust 默认变量是不可变的，需要通过mut设置为可变的
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    print!("The value of x is: {x}");
    print!("\n");
    // const 定义常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let y = 5;
    let y = y + 1;
    // {}定义了一个作用域，y*2只在这个作用域内起作用
    {
        let y = y * 2;
        println!("The value of x in inner scope is: {y}");  // 12
    }
    println!("The value of x is: {y}");  // 6
    let space = "    ";
    let space = space.len();

    // 会报错
    // let mut spaces = "   ";
    // spaces = spaces.len();

}

pub fn data_types_2(){
    let guess: u32= "42".parse().expect("Not a number"); // 必须加u32，否则会报错

    // Scalar Types ---- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
    // iterger types: i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 isize usize 
    // Decimal 98_222   Hex 0xff    Octal 0o77  Binary 0b1111_0000  Byte(u8 only) b'A'

    // Floating-Point Types  IEEE-754 standard
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1
    let remainder = 43 % 5;

    // Boolean Type
    let t = true;
    let f: bool = false;  // with explicit type annotation

    // Character type : char type is four bytes in Rust, include--Chinese,Japanese,Korean,emoji,zero-width spaces
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '🐱';

    // Compound Types: tuple, arrays
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);  // 元组内的数据类型可以不同，不能改变元组大小
    let tup = (501, 6.5, 2);
    let (x, y, z) = tup;  // 解包
    println!("The value of y is: {y}");

    let x1: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x1.0;
    let six_point_four = x1.1;
    let one = x1.2;

    // Array Type
    let a = [1, 2, 3, 4, 5];  // 可以改变数组大小，相同的数据类型
    let months = ["January", "February", "March", "April", "May", "June", "July",
                              "August", "September", "October", "November", "December"];
    let a1 = [3; 6];  // 指定数组长度为6，元素值为3
    let first = a[0];  // 使用索引取值
    let second = a[1];  

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}

pub fn functions_3(x: i32, unit_label: char) {
    // parramenters
    println!("The value of x is: {x}{unit_label}");

    // statement and expressions
    // 在rust中{}包裹的代码最后一行有分号表示语句，没有分号表示表达式
    let y = {
        let x = 3;
        x + 1  // 注意这里不加;号，表示x+1的值会返回给y，这里是表达式有返回值，加上分号表示语句，没有返回值
    };
    println!("The value of y is : {y}");

    // Functions with Return Values
    let x = five();
    println!("five function return value is {x}");
}

pub fn comments_4() {
    // this is a comment
}

pub fn control_flow_5() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let num = 6;
    if num % 4 == 0 {
        println!("num is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is divisible by 2");
    } else {
        println!("num is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let num1 = if condition {5} else {6};  // if condition { 5 } else { "six" };会报错，返回值类型不一致
    println!("The value of number is {num1}");

    // Repetition with Loops
    let mut loop_count = 0;
    loop {
        if loop_count > 10 {
            break;
        }
        println!("again");
        loop_count += 1;
    }
    let mut counter = 0;
    let result = loop {
        if counter == 10 {
            break counter * 10;  // 这里的分号可加可不加  break语句可以中断循环并返回一个值
        }
        counter += 1;
    };
    println!("The result is {result}");

    // Loop Labels to Disambiguate Between Multiple Loops 消除循环嵌套 
    // nested loops 嵌套的循环 使用单引号来生成循环标签，通过break or continue指定标签来中断某个循环
    let mut first_count = 0;
    'counting_first: loop {
        println!("count = {first_count}");
        let mut second_count = 10;
        'counting_second: loop {
            println!("second_count = {second_count}");
            let mut three_count = 0;
            loop {
                println!("three_count = {three_count}");
                if three_count > second_count {
                    println!("loop break second_count={second_count}, three_count={three_count}, first_count={first_count}");
                    break;
                }
                if second_count == three_count && second_count == 5 {
                    println!("counting_second loop break second_count={second_count}, three_count={three_count}, first_count={first_count}");
                    break 'counting_second;
                }
                if first_count == second_count + three_count {
                    println!("counting_first loop break second_count={second_count}, three_count={three_count}, first_count={first_count}");
                    break 'counting_first;
                }
                three_count += 1;
                if three_count == 10 {
                    break;
                }
            }
            second_count -= 1;
            if second_count == 0 {
                break;
            }
        }
        first_count += 1;
    }
    println!("End count = {first_count}");

    // Conditional Loops with while
    let mut loop_num = 3;
    while loop_num != 0 {
        println!("{loop_num}");
        loop_num -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    for ele in a {
        println!("the value is: {ele}");
    }

    // 逆置打印
    for num in (1..10).rev() {
        println!("rev num {num}");
    }

}

fn five() -> i32 {
    10 // 隐式返回
}

pub fn print_test(){
    println!();
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
