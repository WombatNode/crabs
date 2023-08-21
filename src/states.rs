use std::default;

use crate::pets::{Pet, Stats};
use crate::food::Food;

pub type Id = u32;

// A team of pets
pub type Team = [Option<Pet>; 5];

// Represents a side of a battle
#[derive(Clone, Copy)]
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
    next_id: Id,
}

impl PlayerState {
    // Get one side of the battle
    // Will return None if there is no relevant team, ie. trying to get the opposition while in the shop
    pub fn get_team_from_side<'a>(&'a mut self, side: Side) -> Option<&'a mut Team> {
        match (side, &mut self.activity) {
            (Side::A, Activity::Shop) => {
                Some(&mut self.team)
            },
            (Side::A, Activity::Battle{team, opposition}) => {
                Some(team)
            },
            (Side::B, Activity::Shop) => None,
            (Side::B, Activity::Battle {team, opposition }) => {
                Some(opposition)
            },
        }
    }
}

impl PlayerState {
    // Generate a unique id for a pet to be used for individual identification
    // As this project is currently single-threaded, there is no need for atomics
    fn gen_id(&mut self) -> Id {
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



pub enum GameResult {
    Win,
    Loss,
}

