// rust allows you to create a struct resembling like a tuple, in a concise mannner

struct Location(i32, i32);

fn main() {
    // this is still a struct on a stack
    let loc = Location(34, 56);
    println!("struct like a tuple: {}, {}", loc.0, loc.1);
}