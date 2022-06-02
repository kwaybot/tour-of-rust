// loop can break to return a value
// in other words, if we want to return something from the loop, we can use break keyword which acts as a return statement essentially in this case

fn main() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "bruh a message from loop";
        }
    };
    println!("from loop: {}",v);
}