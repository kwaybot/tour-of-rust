// enum can have one or more data types
// when an enum is pattern matched using match, you can bind a variable to name each data value
// enum would have a mem size of its largest element
// each element also has a numeric value that reps which tag it is
// rust's enum is aka tagged union
// combining types to make new types = algebraic types


#![allow(dead_code)] // this line prevents comiler warnings

enum Species { Crab, Octopus, Fish, Clam }
enum PoisonType { Acidic, Painful, Lethal }
enum Size { Big, Small }
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    let ferris = SeaCreature {
        species: Species::Crab,
        name: String::from("Ferris"),
        arms: 2,
        legs: 4,
        weapon: Weapon::Claw(2, Size::Small),
    };

    match ferris.species {
        Species::Crab => {
            match ferris.weapon {
                Weapon::Claw(num_claws, size) => {
                    let size_desc = match size {
                        Size::Big => "big",
                        Size::Small => "small"
                    };
                    println!("ferris is a crab with {} {} claws", num_claws, size_desc);
                },
                _ => println!("ferris is a crab with some other weapon")
            }            
        },
        _ => println!("ferris is some other animal"),
    }
}