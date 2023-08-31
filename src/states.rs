use std::cmp::min;

use rand::seq::{IteratorRandom, SliceRandom, SliceChooseIter};

use crate::battles::get_pet_from_team;
use crate::pets::{Pet, PetDetails};
use crate::food::Food;
use crate::stats::Stats;

pub type Id = u32;

// A team of pets
pub type Team = [Option<Pet>; 5];

// Represents a side of a battle
#[derive(Clone, Copy)]
pub enum Side {
    Player = 0,
    Opposition = 1,
}

impl Side {
    // Get opposing side
    pub fn opposition(&self) -> Side {
        match self {
            Side::Player => Side::Opposition,
            Side::Opposition => Side::Player,
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
    pub fn new() -> Self {
        Self {
            team: todo!(),
            shop: todo!(),
            shop_scaling: todo!(),
            activity: todo!(),
            next_id: todo!(),
        }
    }

    // Get both sides of battle in tuple
    // Teams may be None
    // Returns a tuple where element 0 is the 'players' side, and 1 is the opposition. This doesn't account for in battle stuff, so the perspective functions should be used in those cases
    pub fn get_teams<'a>(&'a mut self) -> [Option<&'a mut Team>; 2] {
        match &mut self.activity {
            Activity::Shop => {
                [Some(&mut self.team), None]
            },
            Activity::Battle{team, opposition} => {
                [Some(team), Some(opposition)]
            },
        }
    }

    // Generate a unique id for a pet to be used for individual identification
    // As this project is currently single-threaded, there is no need for atomics
    fn gen_id(&mut self) -> Id {
        let id = self.next_id;
        self.next_id = id + 1;
        id
    }

    pub fn get_pet<'a>(&'a mut self, id: Id) -> Option<PetDetails<'a>> {
        // Check our team first
        let [team, opposition] = self.get_teams();
        if let Some((pet, position)) = get_pet_from_team(team, id) {
            Some(PetDetails {
                pet,
                side: Side::Player,
                position,
            })
        }
        else if let Some((pet, position)) = get_pet_from_team(opposition, id) {
            Some(PetDetails {
                pet,
                side: Side::Opposition,
                position,
            })
        } else {
            None
        }
    }

    pub fn get_team(&mut self, side: Side) -> Option<&mut Team> {
        self.get_teams()
            .get_mut(side as usize)
            .unwrap()
            .take()
    }

    pub fn get_n_random<'a, P>(&mut self, n: usize, side: Side, predicate: P) -> 
            impl Iterator<Item = PetDetails>  
    where
        P: FnMut(&PetDetails) -> bool,
        P: Copy,
    {
        let mut options: Vec<PetDetails> = match self.get_team(side) {
            Some(team) => {
                team
                .into_iter()
                .enumerate()
                .filter_map(
                    |(position, pet)| pet.as_mut().map(|pet| PetDetails {
                        pet,
                        side,
                        position,
                    })
                    .filter(predicate)
                )
                .collect()
            },
            None => Vec::new(),
        };

        options.shuffle(&mut rand::thread_rng());

        return options.into_iter().take(n);
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

pub fn apply_to_n_in_team<F>(team: &mut Team, mut action: F, n: usize, state: &mut PlayerState) 
where 
    F: FnMut(&mut Pet, &mut PlayerState)
{
    let some_elements: Vec<&mut Pet> = team.into_iter()
        .filter_map(|x| x.as_mut())
        .collect();
    let n = min(some_elements.len(), n);

    let random_pets = some_elements.into_iter()
        .choose_multiple(&mut rand::thread_rng(), n);

    for pet in random_pets {
        action(pet, state);
    }

}

pub enum GameResult {
    Win,
    Loss,
}

