// if no return type is specified for a fn, it returns an empty tuple aka unit
// an empty tuple is repped by ()

fn make_nothin() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothin2() {
    // this fn will return () if nothing is specified to return
}

fn main() {
    let a = make_nothin();
    let b = make_nothin2();

    // printing a debug string for a and b
    // because it's hard to print nothingness
    println!("The value of a: {:?}",a);
    println!("The value of b: {:?}",b);
}