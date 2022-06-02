// conditions dont have parenthesis
// ops: ==, !=, <, >, <=, >=, !, ||, &&
fn main() {
    let x = 34;
    if x < 30 {
        println!("less than 30");
    } else if x == 34 {
        println!("is 34");
    } else {
        println!("greater than 34");
    }
}