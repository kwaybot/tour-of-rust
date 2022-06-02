// for loop iterates over values from any expression that evaluates into an iterator
// iterator is an object that can read items in a list until there are no items left
// .. operator creates an iterator that generates a number from a start number up to but not including the end number
// ..= does the same as .. but includes the end number

fn main() {
    for x in 0..5{
        println!("{}",x);
    }
    for x in 0..=5{
        println!("{}",x);
    }
}