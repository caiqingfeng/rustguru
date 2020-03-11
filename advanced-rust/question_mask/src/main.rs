// 下面的例子提供了一个不使用?符号 以及 一个使用?符号的样例代码.
fn halves_if_even<'a >(i: i32) -> Result<i32, &'a str> {                       // 取数值的二分之一.
    if i % 2 == 0 {
        Ok(i/2)
    } else {
        Err("error")
    }
}

fn not_use_question_mark() {
    let a = 10;                                                                // 把这里改成 9 就会报错.
    let half = halves_if_even(a);
    let half = match half {
        Ok(item) => item,
        Err(e) => panic!(e),
    };
    assert_eq!(half, 5);
}


fn use_question_mark<'a >() -> Result<i32, &'a str> {                          // 这里必须要返回Result
    let a = 10;
    let half = halves_if_even(a)?;                                             // 因为?要求其所在的函数必须要返回Result
    assert_eq!(half, 5);
    Ok(half)                                                                   
}


fn main() {
    not_use_question_mark();
    let _ = use_question_mark();
}