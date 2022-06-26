// when an owner is passed as an argument to a function, ownership is moved to the function parem
// after a move the variable in the original function can no longer be used

// mem details: during the move, the stack memory of the owners value is copied to the function call's parem stack memory

// -----------------------------------------------------------------------------------------------------------------------

struct Foo {
    x: i32,
}

fn do_smth(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    let foo = Foo { x: 42 };
    // foo is moved to do_smth
    do_smth(foo);
    // foo can no longer be used
}