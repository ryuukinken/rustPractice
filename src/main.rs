use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("数字を当たってみてください。");

    let random_number: i32 = rand::thread_rng().gen_range(1..=100);

    // println!("このランダム数字は:{}です。",random_number);
    loop {
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("読み取りできません！");

        let guess: i32 = guess.trim().parse().expect("数字を入力してください！");

        // print!("入力された数字は:{}です。",guess);

        // 乱数と入力された数字を比較する
        match guess.cmp(&random_number) {
            Ordering::Less => println!("TOO SMALL!"),
            Ordering::Greater => println!("TOO BIG!"),
            Ordering::Equal => {
                println!("BINGO!");
                break;
            }
        }
    }
}