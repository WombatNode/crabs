use std::cmp::{Ordering, min};

use rand::seq::IteratorRandom;

use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::{Pet, PetDetails, trigger_action}, actions::{ActionQueue, Action, ActionResolver}, triggers::Trigger, utils::is_living};


pub enum DamageType {
    Snipe,
    Attack,
}

pub fn simulate_battle(mut state: PlayerState) -> GameResult {
    let mut resolver = ActionResolver::new();

    // Add start of battle abilities
    for pet in state.get_all_pets() {
        trigger_action(&mut resolver, pet, Trigger::StartOfBattle);
    }

    resolver.resolve(&mut state);

    // Loop through the pets attacking each other until one side has no pets left
    loop {
        let player_pet = state.get_n_pets(1, Side::Player, is_living, |_| return).next();
        let opposition_pet = state.get_n_pets(1, Side::Opposition, is_living, |_| return).next();

        match (player_pet, opposition_pet) {
            
        }
    }

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

pub fn hurt(damage: u8, source: Id, damage_type: DamageType, 
        target: PetDetails, action_resolver: &mut ActionResolver) 
{
    // Deal with food mitigation

    // If zero damage is done, the pet shouldn't be hurt
    if damage == 0 {
        return;
    }

    // Decrease hp
    let new_hp = target.pet.stats.hp.saturating_sub(damage);
    target.pet.stats.hp = new_hp;

    // Apply hurt trigger to pet
    let trigger = Trigger::Hurt { source, damage_type };
    trigger_action(action_resolver, target, trigger);

    // Check if the pet has been killed
    if new_hp == 0 {
        todo!()
        // Pet is dead
    }
}



