Option
enum Option<T> {
    None,
    Some(T),
}
Option 是Rust的系统类型，用来表示值不存在的可能，这在编程中是一个好的实践，它强制Rust检测和处理值不存在的情况。例如：
fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
find在字符串haystack中查找needle字符，事实上结果会出现两种可能，有（Some(usize))或无（None）。
fn main() {
    let file_name = "foobar.rs";
    match find(file_name, '.') {
        None => println!("No file extension found."),
        Some(i) => println!("File extension: {}", &file_name[i+1..]),
    }
}
Rust 使用模式匹配来处理返回值，调用者必须处理结果为None的情况。这往往是一个好的编程习惯，可以减少潜在的bug。Option 包含一些方法来简化模式匹配，毕竟过多的match会使代码变得臃肿，这也是滋生bug的原因之一。
unwrap
impl<T> Option<T> {
    fn unwrap(self) -> T {
        match self {
            Option::Some(val) => val,
            Option::None =>
              panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
unwrap当遇到None值时会panic，如前面所说这不是一个好的工程实践。不过有些时候却非常有用：
在例子和简单快速的编码中 有的时候你只是需要一个小例子或者一个简单的小程序，输入输出已经确定，你根本没必要花太多时间考虑错误处理，使用unwrap变得非常合适。
当程序遇到了致命的bug，panic是最优选择
map
假如我们要在一个字符串中找到文件的扩展名，比如foo.rs中的rs， 我们可以这样：
fn extension_explicit(file_name: &str) -> Option<&str> {
    match find(file_name, '.') {
        None => None,
        Some(i) => Some(&file_name[i+1..]),
    }
}

fn main() {
    match extension_explicit("foo.rs") {
        None => println!("no extension"),
        Some(ext) =>  assert_eq!(ext, "rs"),
    }
}
我们可以使用map简化：
// map是标准库中的方法
fn map<F, T, A>(option: Option<T>, f: F) -> Option<A> where F: FnOnce(T) -> A {
    match option {
        None => None,
        Some(value) => Some(f(value)),
    }
}
// 使用map去掉match
fn extension(file_name: &str) -> Option<&str> {
    find(file_name, '.').map(|i| &file_name[i+1..])
}
map如果有值Some(T)会执行f，反之直接返回None。