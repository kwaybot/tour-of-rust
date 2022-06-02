// 'break' keyword will escape a loop
fn main() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 21 {
            break;
        }
    }
    println!("value of x: {}",x);
}