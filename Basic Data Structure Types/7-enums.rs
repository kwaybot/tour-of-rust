// enums allow you to create a new type that can have value of several tagged elements
// enum keyword
// match helps handle all possible enum values

#![allow(dead_code)] // this line prevents compiler warnings

enum Species {
    Crab,
    Octopus,
    Fish,
    Clam
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: String,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: String::from("claw"),
    };

    match ferris.species {
        Species::Crab => println!("{} is a Crab", ferris.name),
        Species::Octopus => println!("{} is a Octopus", ferris.name),
        Species::Fish => println!("{} is a Fish", ferris.name),
        Species::Clam => println!("{} is a Clam", ferris.name),
    }
}