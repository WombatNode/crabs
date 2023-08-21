use std::default;

use crate::pets::{Pet, Stats};
use crate::food::Food;


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



#[derive(Default)]
pub struct Team {
    pets: [Option<Pet>; 5],
}

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

pub struct GameState {
    player_state: PlayerState,
    activity: Activity,
    next_id: u32,
}

impl GameState {
    fn gen_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id = id + 1;
        id
    }
}

pub enum Activity {
    Shop,
    Battle {
        team: Team,
        opposition: Team,
    },
}

#[derive(Default)]
pub struct PlayerState {
    team: Team,
    shop: [ShopSlot; 9],
    shop_scaling: Stats,
}

pub struct BattlePerspective<'a> {
    side: Side,
    // If provided, represents which position in the lineup the pet doing the action is in
    pet_position: Option<usize>,
    game_state: &'a mut GameState,
}


pub enum GameResult {
    Win,
    Loss,
}

pub fn simulate_battle(state: BattlePerspective) -> GameResult {

    GameResult::Loss
}