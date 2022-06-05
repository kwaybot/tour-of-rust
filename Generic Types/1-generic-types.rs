// gen types allow us to partially define a struct or enum
// rust generally can infer the final type of any instantiaion
// we can explicitly define the type using ::<T> operator, aka turbofish
// gen types create compile-time created types

// ----------------------------------------------------------------------

// partially defined struct type
struct BagOfHolding<T> {
    item: T,
}

fn main() {
    
    // defining explicitly using turbofish
    let i32_bag = BagOfHolding::<i32> { item: 42 };
    let bool_bag = BagOfHolding::<bool> { item: false };

    // rust can sometimes infer types for generics too
    let float_bag = BagOfHolding { item: 3.14 };

    // Note: never put a bag of holding in a bag of holding in real life
    let bag_in_bag = BagOfHolding {
        item: BagOfHolding { item: "boom!" },
    };

    println!("{} {} {} {}",
            i32_bag.item, bool_bag.item, float_bag.item, bag_in_bag.item.item);
}