pub(crate) fn ownership_practice(){
    // 文字列
    let a = String::from("hello world!");
    let b = a;
    println!("{}",b);
    // println!("{}",a); ※ value borrowed here after move

    // 数字
    let c = 1;
    let d  = c;
    println!("{}",c);
    println!("{}",d); // 数字はスタックに保存されたため、ここでアクセスできる
}

// 戻り値ある関数
pub(crate) fn value_plus_function(x:i64, y:i64) -> i64{
    return x+y;
}
// 戻り値なし関数
pub fn print_hello_world(){
    println!("hello World!")
}