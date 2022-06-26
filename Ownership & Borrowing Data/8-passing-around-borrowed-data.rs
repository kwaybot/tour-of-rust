// rules for referencing:
// 1) rust only allows one mutable reference or multiple non-mutable references but not both
// 2) a reference must never live longer than its owner - removing dangling pointers by default

// ---------------------------------------------------------------------------------------------

struct Foo {
    s: i32,
}

fn do _smth(f: &mut Foo) {
    f.x += 1;
}

fn main () {
    let mut foo = Foo { x: 42 };
    do_smth(&mut foo);
    // because all mutable references are dropped within the func do_smth()
    do_smth(&mut foo);
    // foo is dropped here
}