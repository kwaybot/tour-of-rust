// refs can be used on pieces of refs

struct Foo {
    x: i32,
}

fn do_smth(a: &Foo) -> &i32 {
    return &a.x;
}

fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here, allowing us to create a non-mutable reference
    let y = do_smth(&foo);
    println!("{}", y);
    // y is dropped here
    // foo is dropped here
}