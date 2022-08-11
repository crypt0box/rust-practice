pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    // {
    //     let st4 = String::from("y");
    //     let res2 = get_longest(&st3, &st4);
    //     println!("{}", res2);
    // }
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
}
// ジェネリックライフタイムアノテーション
// 引数で受けとった参照のライフタイムのうち短い方を返り値として適用する
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
