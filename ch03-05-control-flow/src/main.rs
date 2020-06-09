fn main() {
    let number=3;
    if number<5 {
        println!("condition is true")
    } else if number>=5 {
        println!("condition is false")
    }

    let cond=true;
    let number=if cond {
        5
    } else {
        6
//        "six"
//        error[E0308]: `if` and `else` have incompatible types
    };

    println!("The value of the number is:{}",number);

    let mut number=5;
    let res=loop {
        println!("The number is:{}",number);
        number*=2;
        if number>50 {
            break number;
        }
    };
    println!("The result is:{}",res);
//    返回值是80，它可以把break的值给你返回来

    println!("Hello, world!");

    let arr=[1,2,3,4,5];
    for num in arr.iter() {
//        borrow the array with `&` or call `.iter()` on it to iterate over it
        println!("The number is:{}",num);
    }
    for num in &arr {
//        borrow the array with `&` or call `.iter()` on it to iterate over it
        println!("The number is:{}",num);
    }
    println!("Blow!");
}
