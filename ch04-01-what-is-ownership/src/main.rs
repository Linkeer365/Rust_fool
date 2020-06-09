fn main() {
///    Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
///    值有且只有一个所有者。
///    当所有者（变量）离开作用域，这个值将被丢弃。

    let _s = "hello";
    {                      // s 在这里无效, 它尚未声明
        let _s = "hello";   // 从此处起，s 是有效的

        // 使用 s
    }                      // 此作用域已结束，s 不再有效

///   换句话说，这里有两个重要的时间点：
///   当 s 进入作用域 时，它就是有效的。
///   这一直持续到它 离开作用域 为止。


    let mut s=String::from("hello");
/// 这两个冒号（::）是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，
/// 而不需要使用类似 string_from 这样的名字。
    s.push_str(" world");
    println!("{}",s);
    println!("Hello, world!");

    // 堆上元素，克隆语义
    let s=String::from("kksk");
    let ss=s.clone();
    println!("{}",ss);
    println!("{}",s);

    // 堆上元素，移动语义
    let s=String::from("kksk");
    let ss=s;
    println!("{}",ss);
//    println!("{}",s); 被移动所以不存在

    // 栈上元素，直接赋值（或者叫直接拷贝，但不太准确）
    let x=3;
    let y=x;
    println!("x={};y={}",x,y);
    // 也支持克隆语义
    let y=x.clone();
    println!("x={};y={}",x,y);

    /// Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上（第十章详细讲解 trait）。
    /// 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。
    /// Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait
    /// 作为一个通用的规则，任何简单标量值的组合可以是 Copy 的，
    /// 不需要分配内存或某种形式资源的类型是 Copy 的

    let s=String::from("hello");
    take_ownership(s);

    // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

//    println!("{}",s);

    let x=5;
    make_copy(x);

    // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x

    println!("x={}",x);

    let s1 = gives_ownership();         // gives_ownership 将返回值
    // 移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);
    // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
}
// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
// 所以不会有特殊操作

// 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
// 所以什么也不会发生。s1 移出作用域并被丢弃

fn take_ownership(some_string: String){
    println!("{}",some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn make_copy(some_integer: i32){
    println!("{}",some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

fn gives_ownership() -> String {
    // gives_ownership 将返回值移动给
    // 调用它的函数
    let some_string = String::from("hello"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
