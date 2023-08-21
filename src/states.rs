use std::default;

use crate::pets::{Pet, Stats};
use crate::food::Food;

// Represents a side of a battle
pub enum Side {
    A,
    B,
}

impl Side {
    // Get opposing side
    fn opposition(&self) -> Side {
        match self {
            Side::A => Side::B,
            Side::B => Side::A,
        }
    }
}


// A team of pets
#[derive(Default)]
pub struct Team {
    pets: [Option<Pet>; 5],
}

// Represents a slot in the shop. Can either be empty, or contain a food or pet
#[derive(Default)]
enum ShopSlot {
    #[default]
    None,
    Pet {
        pet: Pet,
        cost: u8,
        frozen: bool,
    },
    Food {
        food: Food,
        cost: u8,
        frozen: bool,
    },
}

// Represents the entirety of the game's state from a particular player's perspective
pub struct PlayerState {
    team: Team,
    shop: [ShopSlot; 9],
    shop_scaling: Stats,    
    activity: Activity,
    next_id: u32,
}

impl PlayerState {
    // Generate a unique id for a pet to be used for individual identification
    // As this project is currently single-threaded, there is no need for atomics
    fn gen_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = id + 1;
        id
    }
}

// Represents what the player is currently doing,
// ie. are they in the shop, or in battle
// If they are in battle, then new teams should be created for the battle simulation
pub enum Activity {
    Shop,
    Battle {
        team: Team,
        opposition: Team,
    },
}

// A wrapper around gamestate to be used in battle
// provides generic ways to interact with the game's state
pub struct BattlePerspective<'a> {
    side: Side,
    // If provided, represents which position in the lineup the pet doing the action is in
    pet_position: Option<usize>,
    game_state: &'a mut PlayerState,
}


pub enum GameResult {
    Win,
    Loss,
}

pub fn simulate_battle(state: BattlePerspective) -> GameResult {

    GameResult::Loss
}