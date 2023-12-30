use std::io;
use rand::Rng;

fn main() {
    println!("hello world!!!");
    println!("数字を入力してください。");

    let random_number:i32 = rand::thread_rng().gen_range(1..=100);

    println!("このランダム数字は:{}です。",random_number);

    let mut guess:String = String::new();

    io::stdin().read_line(&mut guess).expect("読み取りできません！");

    print!("入力された数字は:{}です。",guess);

}