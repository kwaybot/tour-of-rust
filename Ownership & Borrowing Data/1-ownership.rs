// instantiating a type and binding it to a variable name creates a mem resource
// the resources will be validated by the compiler through its whole lifetime
// bound variable is the resource's owner

// ------------------------------------------------------------------------------

#[derive(Debug)]
struct Foo {
    x: i32,
}

fn main () {
    // here we're instantiating a struct and binding it to variable
    // which will create a mem resource
    let foo = Foo { x: 42 };
    // foo is the owner
    println!("{:?}",foo);
}