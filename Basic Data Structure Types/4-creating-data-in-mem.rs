// when we instantiate a struct, program creates associated field data side by side in memory
// instantiation happens when we specify all the fields
// struct fields are accessed using a dot operator .
// read only data is stored in Data Mem region
// modifiable data is placed in Heap region


// Behind the scenes:
// 1) func call String::from 
// 2) creates a struct String in Stack region 
// 3) creates a mem on Heap region where it can be modified 
// 4) storing a reference to that mem location on heap in String struct on stack

struct SeaCreature {
    animal_type: String,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    // SeaCreature's data is on stack
    let ferris = SeaCreature {
        // String struct is also on stack but holds ref to data on heap
        animal_type: String::from("crab"),
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    let sarah = SeaCreature {
        animal_type: String::from("octopus"),
        name: String::from("Sarah"),
        arms: 8,
        legs: 0,
        weapon: String::from("brain"),
    };

    println!("{} is a {}. They have {} arms, {} legs and a {} weapon.",
            ferris.name, ferris.animal_type, ferris.arms, ferris.legs, ferris.weapon);
    
    println!("{} is a {}. They have {} arms, {} legs and a {} weapon.",
            sarah.name, sarah.animal_type, sarah.arms, sarah.legs, sarah.weapon);
}