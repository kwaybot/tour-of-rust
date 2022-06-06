// working with Option / Result can be tedious when writing some quick code
// both have a built-in func called 'unwrap'
// unwrap can be used to get values in a quick and dirty manner
// unwrap will 
// 1) get the value inside Option / Result
// 2) if the enum is of type None / Err, it throws panic!

// ---------------------------------------------------------------------------

fn do_smth_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(23.5)
    } else {
        Err(String::from("not correct input :/"))
    }
}

fn main() -> Result<(), String> {
    // concise but assumptive and gets ugly fast
    let v = do_smth_that_might_fail(42).unwrap();
    println!("found {}", v);

    // this will panic!
    let v = do_smth_that_might_fail(34).unwrap();
    println!("found {}", v);

    Ok(())
}


// ---------------------------------------------------------------------------

// Output:
/* [kwaybot@kbot Generic Types]$ ./7-unwrap-func-handling 
    found 23.5
    thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "not correct input :/"', 7-unwrap-func-handling.rs:24:41
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
*/