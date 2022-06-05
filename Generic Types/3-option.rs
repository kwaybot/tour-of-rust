// Option is a built-in generic enum that allows us to represent nullable values without using null
// very commonly used enum
// can be created anywhere with the enum variants Some and None

// --------------------------------------------------------------------------------------------------

struct BagOfHolding<T> {
    // parameter type T can be handed to others
    item: Option<T>,
}

fn main() {
    // since we are creating a bag holding nothing we have to give it some type explicitly
    // cuz rust cannot infer the type from nothing
    let i32_bag = BagOfHolding::<i32> { item: None };

    if i32_bag.item.is_none() {
        println!("there's nothing in the bag!");
    } else {
        println!("there's something in the bag!");
    }

    let i32_bag = BagOfHolding::<i32> { item: Some(34) };

    if i32_bag.item.is_some() {
        println!("there's something in the bag!");
    } else {
        println!("there's nothing in the bag!");
    }

    // match helps us handle all the cases elegantly
    match i32_bag.item {
        Some(v) => println!("found {} in the bag", v),
        None => println!("found nothing in the bag"),
    }
}