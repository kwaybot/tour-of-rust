// result enum is v common in rust
// ? operator for working with result enum

/*
do_smth_that_might_fail()?
*/

// the above statement is equivalent of a match handling the result enum

// -----------------------------------------

fn do_smth_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 34 {
        Ok(91.21)
    } else {
        Err(String::from("this is not the correct number"))
    }
}

fn main() -> Result<(), String> {
    // ? operator saves writing a lot of code
    let v = do_smth_that_might_fail(35)?;
    println!("found {}", v);
    Ok(())
}