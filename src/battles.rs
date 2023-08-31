use std::cmp::{Ordering, min};

use rand::seq::IteratorRandom;

use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::Pet, actions::{ActionQueue, ActiveAction, FinalAction, ActionResolver}, triggers::Trigger};


pub enum DamageType {
    Snipe,
    Attack,
}

pub fn simulate_battle(mut state: PlayerState) -> GameResult {
    let perspective = Perspective::new(&mut state);

    GameResult::Loss
}


fn get_pet_index_from_id(team: &mut Team, id: Id) -> Option<usize> {
    team.into_iter()
        .enumerate()
        .find_map(|(index, pet)| {
            match pet {
                Some(pet) if pet.id == id => {
                    Some(index)
                },
                _ => None,
            }
        })
} 

pub fn get_pet_from_team(team: Option<&mut Team>, id: Id) -> Option<(&mut Pet, usize)> {
    team?.into_iter()
        .enumerate()
        .find_map(|(position, pet)| {
            match pet {
                Some(pet) if pet.id == id => {
                    Some((pet, position))
                },
                _ => None,
            }
        })
} 

// A wrapper around gamestate to be used in battle, or whenever actions need to be resolved in some order
// provides generic ways to interact with the game's state
pub struct Perspective<'player, 'active> {
    pub state: &'player mut PlayerState,
    pub side: Side,
    // If provided, represents which position in the lineup the pet doing the action is in
    pub pet_id: Option<Id>,
    pub action_resolver: ActionResolver<'active>,

}

impl <'player, 'active> Perspective<'player, 'active> {
    // Create a new Perspective based on the side of the battle that we are currently on
    fn new(state: &'player mut PlayerState) -> 
        Perspective<'player, 'active> 
    {
        Perspective { 
            state, 
            side: Side::Player,
            pet_id: None,
            action_resolver: ActionResolver::new(),
        }
    }

    // fn find_pet(&mut self, pet_id: Id) {
    //     // We shouldn't be trying to find a pet from the opposing team while in the shop
    //     let team = self.state.get_team_from_side(self.side).unwrap();
    //     // Find the pet
    //     self.pet_position = get_pet_index_from_id(team, pet_id);        
    // }

    pub fn get_team(&mut self) -> Option<&mut Team> {
        self.state
            .get_teams()
            .get_mut(self.side as usize)
            .unwrap()
            .take()
    }

    pub fn get_opposition(&mut self) -> Option<&mut Team> {
        self.state
            .get_teams()
            .get_mut(self.side.opposition() as usize)
            .unwrap()
            .take()
    }

    // pub fn get_current_pet(&mut self) -> Option<&mut Pet> {
    //     let pet_id = self.pet_id?;
    //     let (pet, _position) = self.state.get_pet(pet_id)?;
    //     Some(pet)
    // }

    // pub fn apply_to_n_foes<F>(&mut self, action: F, cmp: fn(Pet, Pet) -> Ordering) 
    pub fn apply_to_n_foes<F>(&mut self, mut action: F, n: usize) 
    where 
        F: FnMut(&mut Pet, &mut Self)
    {
        let team: &mut [Option<Pet>; 5] = self.get_team().unwrap();
        let some_elements: Vec<&mut Pet> = team.into_iter()
            .filter_map(|x| x.as_mut())
            .collect();
        let n = min(some_elements.len(), n);

        let random_pets = some_elements.into_iter()
            .choose_multiple(&mut rand::thread_rng(), n);

        for pet in random_pets {
            action(pet, self);
        }

    }
}

fn hurt(damage: u8, source: Id, damage_type: DamageType, 
        target: &mut Pet, perspective: &mut Perspective) 
{
    // Deal with food mitigation

    // If zero damage is done, the pet shouldn't be hurt
    if damage == 0 {
        return;
    }

    // Decrease hp
    let new_hp = target.stats.hp.saturating_sub(damage);
    target.stats.hp = new_hp;

    // Apply hurt trigger to pet
    let trigger = Trigger::Hurt { source, damage_type };




    // Check if the pet has been killed
    if new_hp == 0 {
        todo!()
        // Pet is dead
    }
}



