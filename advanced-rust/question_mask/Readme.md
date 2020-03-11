// 什么是问号操作符?
// 参考: https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html
// 参考: https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about


// 由于Rust中没有Exception异常处理的语法,
// Rust只有panic报错, 并且panic不允许被保护, 因为没有提供 try 这种语法.
// Rust的异常处理是通过 Result 的 Ok 和 Err 成员来传递和包裹错误信息.
// 然而错误信息的处理一般都是要通过match来对类型进行比较, 所以很多时候
// 代码比较冗余, 通过?符号来简化Ok和Err的判断.
