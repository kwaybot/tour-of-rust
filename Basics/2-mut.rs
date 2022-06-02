fn main() {
    // by default, variables are immutable in rust

    let mut x = 10; // declaring variable x as mutable using mut keyword
    println!("{}", x);
    x = 1;
    println!("{}", x);
}