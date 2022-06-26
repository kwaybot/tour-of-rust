// using &mut references, you can set the owner's value using the * operator
// get a copy of an owned value using the * operator

// ---------------------------------------------------------------------------

fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f;   // getting a copy of the owner's value
    *f = 13;        // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);
}