fn main() {

    let x = 32; // by default rust infers i32 - signed 32 bit integer

    let a = 12u8; // explicitly binding variable a with a value of 12 of type u8 - unsigned 8 bit integer

    let b = 9.8; // by default rust infers f64 - 64 bit floating point

    let c = 10.9f32; // explicilty binding variable c with a value of 10.9 of type f32 -32 bit floating point

    let bv = true; // type - boolean value - true / false

    let t = (13, false); // type tuple, fixed sequence of values on stack

    let sentence = "hello, world!";

    println!(
        "{} , {} , {} , {} , {} , {} , {}, {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}