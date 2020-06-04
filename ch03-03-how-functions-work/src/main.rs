fn main(){
    println!("main");
    another_function();
    print_a_value(5);
    expression_and_statement();
    let x=function_return_value();
    println!("The x value is:{}",x);
    let x=function_expression_value();
    println!("The x value is:{}",x);
}

fn another_function(){
    //     Rust 代码中的函数和变量名使用 snake case 规范风格。
    // 在 snake case 中，所有字母都是小写并使用下划线分隔单词
    println!("another function");
}

fn print_a_value(x: i32){
    println!("The value is:{}",x);
}

fn expression_and_statement() -> () {
//    let x=5;
    let y={
        let z=11;
        z+1
    };
    println!("y is:{}",y);
}

fn function_return_value() -> i32 {
    let x=5;
    return x;
}

fn function_expression_value() -> i32 {
    let x=5;
    x
}