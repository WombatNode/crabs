use crate::pets::Pet;
use crate::food::Food;

pub struct Team {
    pets: [Option<Pet>; 5],
}

enum ShopSlot {
    None,
    Pet {
        pet: Pet,
    },
    Food {
        food: Food,
        cost: u8,
    },
}

pub struct State {
    team: Team,
    shop: [ShopSlot; 9]
}

pub struct BattlePerspective<'a> {
    team: &'a mut Team,
    opposition: &'a mut Team,
}


pub enum GameResult {
    Win,
    Loss,
}

pub fn simulate_battle(state: BattlePerspective) -> GameResult {

    GameResult::Loss
}