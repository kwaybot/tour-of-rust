// main has capability of returning a Result enum

fn do_smth_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 34 {
        Ok(9.121)
    } else {
        Err(String::from("this is not the right number"))
    }
}

// main returns no value, but could return an error
fn main() -> Result<(), String> {
    let result = do_smth_that_might_fail(34);

    // match handles all the cases elegantly
    match result {
        Ok(v) => println!("found {}", v),
        Err(e) => {
            // handle this error gracefully
            // by returning a new error from main explaining what happened
            return Err(String::from("something went wrong in main!"))
        },
    }
    
    Ok(())
}