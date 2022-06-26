// even though rust doesnt always show it in code, compiler understands the lifetime of every variable
// it will attempt to validate that a reference never exists longer than its owner

// funcs can be explicit by paremeterizing the function signature with symbols that help identify 
// which parameters and return values share the same lifetime

// lifetime specifiers always start with a '
// eg: 'a, 'b, 'c

// -----------------------------------------------------------------------------------------------------

struct Foo {
    x: i32,
}

// parem foo and return value share the same lifetime
fn do_smth<'a>(foo: &'a Foo) -> &'a i32 {
    return &foo.x;
}

fn main () {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here, allowing us to create a non-mutable reference
    let y = do_smth(&foo);
    println!("{}", y);
    // y is dropped here
    // foo is dropped here
}