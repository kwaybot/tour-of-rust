// null keyword in other langs = None keyword in rust

enum Item {
    Inventory(String),
    // None represents the absence of an item
    None,
}

struct BagOfHolding {
    item: Item,
}