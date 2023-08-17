mod abilities;
mod pets;

use pets::Pet;

enum Food {
    Apple,
}

enum ShopSlot {
    None,
    Food {
        Food: Food,
        Cost: u8,
    },
}

fn main() {
    println!("Hello, world!");
}
