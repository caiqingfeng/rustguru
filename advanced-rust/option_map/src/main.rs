fn foo() -> Option<u32> {
    Some(10)
}

fn foo2() -> Option<u32> {
    None
}

fn stringify(x: u32) -> String { format!("error code: {}", x) }
fn str2(x: &str) -> String { format!("error code: {}", x) }

fn main() {
    // println!("Hello, world!");
    println!("mapped {}", foo().map(|i| i+1).unwrap());
    println!("mapped_or {}, {}", foo().map_or(33, |i| i+1), foo2().map_or(33, |i| i+1));

    let x: Result<_, &str> = Ok("foo");
    assert_eq!(x.map(|v| v.len()).unwrap(), 3);
    assert_eq!(x.map_or(42, |v| v.len()), 3);
    assert_eq!(x.map_err(str2), Ok("foo"));
    assert_eq!(x.map(|v| v.len()).iter().next().unwrap(), &3);

    let y: Result<&str, _> = Err("bar");
    assert_eq!(y.map_or(42, |v| v.len()), 42);

    let z: Result<u32, u32> = Err(13);
    assert_eq!(z.map_err(stringify), Err("error code: 13".to_string()));
}
