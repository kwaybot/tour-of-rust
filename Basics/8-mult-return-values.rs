// Multiple Return Values
// fns can return multiple values by returning a tuple of values
// tuple elements can be referenced by their index numbers

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y,x);
}

fn main() {
    let result = swap(123,321);
    println!("{} {}", result.0, result.1);

    //destructuring the tuple in two variable names
    let (a,b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}