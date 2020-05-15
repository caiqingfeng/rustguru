pub struct testapi {

}

pub trait chainapi {
    fn foo();
}

impl chainapi for testapi {
    fn foo() {}
}

mod pool;
pub use self::pool::Fullchain;

pub struct testapi2{

}
impl Fullchain for testapi2 {
    fn bar() {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
