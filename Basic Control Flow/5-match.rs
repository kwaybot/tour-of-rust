// match is rust's version of implmenting switch statement
// match is exhaustive so all cases must be handled
// matching is usually combined with destructuring; most common pattern in rust

fn main() {
    let x = 68;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found one or two");
        }
        // we can match against ranges of values
        3..=9 => {
            println!("found a num between 3 to 9 incl");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("could'nt find the match!");
        }
    }
}