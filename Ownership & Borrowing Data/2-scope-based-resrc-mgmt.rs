// rust uses the end of scope to deconstruct and deallocate a resource
// term for decon and dealloc is drop
// mem details:
//      1) rust doesn't have garbage collection
//      2) aka Resource Acquisition Is Initialization (RAII) in C++

// -----------------------------------------------------------------------

struct Foo {
    x: i32,
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 34 };

    println!("{}",foo_a.x);
    println!("{}",foo_b.x);
    // foo_b is dropped here
    // foo_a is dropped here
}