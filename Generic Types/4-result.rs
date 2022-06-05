// Result is a generic enum
// allows us to return a value that has the possibility of failing
// idiomatic way of handling errors in rust
// this enum can be created aywhere with enum variants Ok and Err

fn do_smth_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 34 {
        Ok(9.121)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() {
    let result = do_smth_that_might_fail(34);

    // match handles all the cases elegantly
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => println!("Error: {}", e),
    }
}