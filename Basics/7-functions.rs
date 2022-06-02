// unit that takes input and gives output
// takes zero / more parameters
// function names are always in snake_case

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x -y
}

fn main() {
    println!("3 + 2 = {}", add(3,2));
    println!("3 - 2 = {}", subtract(3,2));
}