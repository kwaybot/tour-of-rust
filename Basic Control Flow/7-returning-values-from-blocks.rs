// all the blocks have unique way of returning values in Rust
// last statement which is an expression without a ';', rust will return it as a value from the block
// allows if statement to operate like a concise ternery expression

fn example() -> i32 {
    let x = 34;
    // rust's ternary expression
    let v = if x < 34 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hotdog";
    let result = match food {
        "hotdog" => "is hotdog",
        //braces are optional if it's a single return expression
        _ => "is not hotdog"
    };
    println!("id'ing food: {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // the idiomatic way of returning a value from fn in rust
    v+5
}

fn main() {
    println!("from function: {}", example());
}