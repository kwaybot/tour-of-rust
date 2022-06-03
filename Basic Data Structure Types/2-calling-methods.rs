// methods are functions associated with a specific data type
// static methods - belong to a type itself, called using :: operator
// instance methods - belong to instance of a type, called using . operator

fn main() {
    // using a static method to create an instance of String
    let s = String::from("Hello, World!");
    // using a method on the String instance s
    println!("{} is {} charecters long", s, s.len());
}