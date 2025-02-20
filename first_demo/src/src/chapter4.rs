use std::io;
use std::fmt::Arguments;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;


// Understanding Ownership

// 1 What is Ownership
/*
    Stack and Heap: 栈和堆
    Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data
    that location is always at the top of the stack. Comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.
    stack: 后进先出，由编译器自动管理，速度快，支持静态和动态分配，是机器系统提供的数据结构，计算机会在底层对栈提供支持：分配专门的寄存器存放栈的地址，压栈出栈都有专门的指令执行，这就决定了栈的效率比较高
    heap: 先进先出，由程序员手动管理，速度较慢，只支持动态分配，堆则是 C/C++ 函数库提供的，它的机制是很复杂的，例如为了分配一块内存，库函数会按照一定的算法（具体的算法可以参考数据结构/操作系统）在堆内存中搜索可用的足够大小的空间，如果没有足够大小的空间（可能是由于内存碎片太多），就有可能调用系统功能去增加程序数据段的内存空间，这样就有机会分到足够大小的内存，然后进行返回。显然，堆的效率比栈要低得多
    rules:
        Each value in Rust has an owner.
        There can only be one owner at a time.
        When the owner goes out of scope, the value will be dropped.
*/
pub fn chapter4_1(){
    // Variable Scope----A variable and the scope in which it is valid
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
    /*
        let s1 = String::from("hello");
        let s2 = s1; // Rust considers s1 as no longer valid

        println!("{s1}, world!"); // 这里会报错
     */
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;  // x还是合法的
    println!("x = {x}, y = {y}");

    let some_string = gives_ownership();  // rust使用表达式返回值，需要提前返回的情况可以使用return返回
    let (s_ret, length) = calculate_length(some_string);
    println!("The length of '{s_ret}' is {length}.");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len();
    (s, length)
}


// References and Borrowing
pub fn chapter4_2(){
    let s1 = String::from("hello");
    let len = calculate_length_p(&s1);
    println!("The length of '{s1}' is {len}");
    let mut s = String::from("hello");
    let change_s = change(&mut s);
    println!("s = {s}, change_s = {change_s}");
    // 变量最多借用一次，借用以后需要使用才能再次借用
    // 使用作用域可以做到引用/借用多次
    let mut ss = String::from("two or more borrow");
    {
        let b1 = &mut ss;
    }
    let b2 = &mut ss;
    // 不能对同一个值同时进行可变和不可变引用, 可以通过使用可变或不可变引用规避
    /*
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        let r3 = &mut s; // big problem
     */
    let mut sm = String::from("sm");
    let r1 = &sm;
    let r2 = &sm;
    println!("r1 = {r1} and r2 = {r2}");
    let r3 = &mut sm;
    println!("r3 = {r3}");

    // Dangling References


}

fn calculate_length_p(s: &String) -> usize {
    s.len()
}

fn change(some_str: &mut String) -> String{
    some_str.push_str(", world");  // 直接改变变量s
    some_str.clone()
}

pub fn chapter4_3(){
    // The Slice Type
    let mut s = String::from("hello, world");
    let s_len = first_word(&s);
    println!("s length is {s_len}");
    s.clear();

    // String Slice  前闭后开，超出索引长度会获取不到值，不会报错
    /*
        let len = s.len();
        let s1 = &s[0..len];
        let s2 = &s[..];
        let s3 = &s[..5];
        let s4 = &s[0..5];
        let s5 = &s[4..];
        let s6 = &s[4..len];
     */
    let s = String::from("String Slice");
    let s1 = &s[0..5];
    let s2 = &s[6..10];
    println!("s1 = {s1}, s2 = {s2}");
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
    
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word2(s: &String) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}




