// fixed length collection of elements of same type
// data type is [T:N], T = elements' type and N = fixed length known at compile time
// elements can be retrived using [x] operator where x is index

fn main() {
    let nums: [i32; 3] = [1,2,3];
    println!("{:?}", nums);
    println!("{}", nums[2]);
}