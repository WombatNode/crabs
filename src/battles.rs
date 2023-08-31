use std::cmp::{Ordering, min};

use rand::seq::IteratorRandom;

use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::Pet, actions::{ActionQueue, ActiveAction, FinalAction, ActionResolver}, triggers::Trigger};


pub enum DamageType {
    Snipe,
    Attack,
}

pub fn simulate_battle(mut state: PlayerState) -> GameResult {

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

fn hurt(damage: u8, source: Id, damage_type: DamageType, 
        target: &mut Pet, state: &mut PlayerState) 
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



