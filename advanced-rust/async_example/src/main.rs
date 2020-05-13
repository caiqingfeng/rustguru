// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheudling
// multiple futures onto the same thread.
use futures::executor::block_on;
use futures::{Future, TryFutureExt};
use futures::future::ok;
use std::error::Error;

async fn hello_world() {
    println!("hello, world!");
}

// fn my_fut() -> impl Future<Item = u32, Error = Box<std::error::Error>> {
//     ok(100)
// }

fn main() {
    let future = hello_world(); // Nothing is printed
    // hell_world();
    block_on(future); // `future` is run and "hello, world!" is printed
    // let f = ok::<_, ()>(String::from("hello"));
    // // let mut reactor = Core::new().unwrap();
    //
    // let retval = block_on(my_fut());
    // println!("{:?}", retval);
}
