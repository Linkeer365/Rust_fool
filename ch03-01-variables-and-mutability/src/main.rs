fn main() {
    {
        const MAX_VALUE: i32=100;
        println!("The Max Value is:{}",MAX_VALUE);
//        const MAX_VALUE:u32=100.0;
        // 常量不能被shadowing
    }
    // 不在作用域之内，常量不能access
//    println!("The Max Value is:{}",MAX_VALUE);
    let mut x =5;
    println!("The x value is {}",x);
    x=6;
    // x=6.5 mut变量可以改变值，但不能改变类型
    println!("The x value is {}",x);
    let spaces="    ";
    // 变量名复用，也叫隐藏shadowing
    println!("The spaces value is:{}",spaces);
    let spaces=spaces.len();
    println!("The spaces value is:{}",spaces);
}
