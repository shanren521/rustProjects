// use std::io::{stdout, BufWriter};
use ferris_says::say;
use std::io;
use std::fmt::Arguments;
use std::io::Write;
use rand::Rng;
use std::cmp::Ordering;

mod chapter3; // 使用mod声明模块
use chapter3::variables_mutability_1; // 通过use导入函数
use chapter3::{print_test, data_types_2, functions_3, control_flow_5};

mod chapter4;
use chapter4::{chapter4_1, chapter4_2, chapter4_3};

mod chapter5;
use chapter5::{chapter5_1, chapter5_2, chapter5_3};


macro_rules! my_macro_main {
    () => {
        println!("Guess the number!");
        println!("Please input your guess.");

        print!("Please input your guess: "); // 会在stdin 和println!之间输出, 需要刷新进打印区
        io::stdout().flush().expect("Failed to flush stdout");  // 将print刷新进打印区
        let mut guess = String:: new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your guess: {}", guess);
    };
}

fn my_fn_main(args: Arguments) {
    let cus_stdout = io::stdout();
    let mut handle = cus_stdout.lock();
    // 写入的三种方式
    writeln!(handle, "writeln!: {}", args).expect("Failed to write to stdout");  // 必须导入use std::io::Write 会自动换行
    handle.write_fmt(format_args!("handle.write_fmt: {}\n", args)).expect("Failed to write to stdout");  // 需要手动添加换行
    std::io::Write::write_fmt(&mut handle, format_args!("std::io::Write::write_fmt: {}\n", args)).expect("Failed to write to stdout");  // 需要手动添加换行
}

fn generation_random_number() {
    println!("Guess the number!");
    
    // let guess: u32 = match guess.trim().parse() {  // 将用户输入的字符串转为int32整型，进行比较否则类型不匹配
    //     Ok(num) => num,
    //     Err(_) => {
    //         println!("Please enter a valid number.");
    //         return;
    //     }
    // };
    
    // let secret_number = rand::thread_rng().gen_range(1..=100).to_string();  // 这里可以添加to_string转为字符串与用户输入进行比较
    // let guess: u32 = guess.trim().parse().expect("Please type a number!"); 也可以通过这种方式将字符串转为int进行比较
    let secret_number = rand::thread_rng().gen_range(1..=10);  // 这里可以添加to_string转为字符串与用户输入进行比较
    // println!("The secret number is: {secret_number}");
    loop {
        print!("Please input your guess: ");
        std::io::stdout().flush().expect("Failed to flush stdout!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read from stdin!");
        println!("Your guess is: {guess}");
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {  // 避免输入非数字导致报错
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win.");
                break;
            }
        }
    }
    
}

fn chapter_3() {
    variables_mutability_1();
    print_test();
    data_types_2();
    functions_3(5, 'h');
    control_flow_5();
}

fn chapter_4() {
    chapter4_1();
    chapter4_2();
    chapter4_3();
}

fn chapter_5() {
    chapter5_1();
    chapter5_2();
    chapter5_3();
}

fn main() {
    let stdout = io::stdout();
    let message = String::from("Hello, World Rustaceans!");
    let width = message.chars().count();
    let mut writer = io::BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
    // my_macro_main!();
    my_fn_main(format_args!("Hello, {}!", "fn_main"));
    my_fn_main(format_args!("The answer is: {}", 42));
    // generation_random_number();
    // chapter_3();
    // chapter_4();
    chapter_5();
}
