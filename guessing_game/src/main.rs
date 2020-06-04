use std::io;

fn main(){
    let mut cc=vec![1,2,3];
    println!("guessing one number!");
    println!("Please input your guess!");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Fail to read line!");
    println!("Your guess: {}", guess);
}
