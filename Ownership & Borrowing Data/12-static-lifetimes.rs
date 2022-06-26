// static var is a mem resource created at compile-time that exists through a program's start to finish
// stat vars must have their types explicitly defined

// static lifetime is a mem resource that lasts indefinitely to the end of a program
// some static lifetime resources can be also created at runtime

// static lifetimes have a special lifetime specifier - 'static

// 'static resources will never be dropped until the end of the control flow

// static lifetime resources must contain references that are 'static, anything else might get dropped or would not live long enough

// modifying static variables is inherently dangerous since they can be read from anyone which
// might introduce a data race.

// rust allows the use of unsafe { ... } blocksto perform some ops the compiler cannot make memory garuntees about

// ----------------------------------------------------------------------------------------------------------------------------------

static PI: f64 = 3.1415;

fn main () {
    // static variables can also be scoped to a function
    static mut SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // you can break some rules, but must be explicit
    unsafe {
        // we can set SECRET to a string literal becuase it also 'static
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}