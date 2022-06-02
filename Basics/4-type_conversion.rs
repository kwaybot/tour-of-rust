// rust requires explicitness with numeric types
// rust makes numeric type conversions very easy with the 'as' keyword
fn main() {
    let a = 9u8;
    let b = 10u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}