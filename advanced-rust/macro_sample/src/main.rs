use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use sp_debug_derive::RuntimeDebug;

#[derive(HelloMacro)]
#[derive(RuntimeDebug)]
struct Pancakes(u64, u64);

fn main() {
    Pancakes::hello_macro();
    println!("{:?}", Pancakes(0, 0));
}

#[test]
fn should_display_proper_debug() {
	assert_eq!(
		format!("{:?}", Pancakes(1, 1)),
		"Pancakes(1, 1)"
	);
}
