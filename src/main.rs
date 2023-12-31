use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
   // guess_number();
    data_types();
}

fn guess_number(){
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

fn data_types(){
    // let 変数名:型 = 値; 例：let i8:i8 = -1;
    // 符号付きバリアントは、-(2^n - 1)以上2^n - 1 - 1以下の数値
    // 符号なしバリアントは、0以上2n - 1以下
    // 範囲-(2^7)から2^7 - 1まで
    let i8:i8 = -1;
    // 範囲0から2^8まで
    let u8:u8 = 1;
    // 範囲-(2^15)から2^15 - 1まで
    let i16:i16 = -16;
    // 範囲0から2^16まで
    let u16:u16 = 16;
    // 範囲-(2^31)から2^31 - 1まで
    let i32:i32 = -32;
    // 範囲0から2^32まで
    let u32:u32 = 32;
    // 範囲-(2^63)から2^63 - 1まで
    let i64:i64 = -64;
    // 範囲0から2^64まで
    let u64:u64 = 64;
    //sizeとusize型は、プログラムが動作しているコンピュータの種類に依存します
    //64ビットアーキテクチャなら、64ビットですし、32ビットアーキテクチャなら、32ビットになります
    let i_size:isize = 128;
    let u_size:usize = 128;

    println!("これらの数字は:{},{},{},{},{},{},{},{},{},{}",i8,u8,i16,u16,i32,u32,i64,u64,i_size,u_size);
}