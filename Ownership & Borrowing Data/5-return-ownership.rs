// ownership can also be returned from a function

// --------------------------------------------------

struct Foo {
    x: i32,
}

fn do_smth() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
}

fn main() {
    let foo = do_smth();
    println!("{}",foo.x);
    // foo becomes the owner
    // foo is dropped because of the end of fucntion scope
}