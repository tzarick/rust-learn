fn main() {
    #[derive(Debug)]
    struct Pirate {
        name: String,
        age: i32,
        ship: String,
        active: bool,
    }

    let pirate = Pirate {
        name: String::from("Jack"),
        age: 32,
        ship: String::from("Blackbeard"),
        active: true,
    };

    let pirate2 = Pirate {
        name: String::from("James"),
        ship: String::from("Blackbeard2"), // if we replace all values explicitly that do not implement "Copy" trait, we can still use pirate. Otherwise, pirate will have "partially moved" to pirate2 which means we can't use pirate anymore
        ..pirate
    };

    println!("{:?}", pirate);
}
