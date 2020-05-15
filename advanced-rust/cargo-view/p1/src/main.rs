use p2::*;

fn foo<P: chainapi>(f: P) {

}

fn bar<P: p2::Fullchain>(f: P) {

}

fn main() {
    let f = testapi{};
    foo(f);
    let f = testapi2{};
    bar(f);
}
