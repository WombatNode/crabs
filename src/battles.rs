use std::cmp::{Ordering, min};

use rand::{seq::IteratorRandom, Rng};

use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::{Pet, PetDetails, trigger_action}, actions::{ActionQueue, Action, ActionResolver}, triggers::Trigger, utils::is_living, food::Food};

#[derive(Clone, Copy)]
pub enum DamageType {
    Snipe,
    Attack,
    Instakill,
}

fn attack(source: PetDetails, target: PetDetails, action_resolver: &mut ActionResolver) {
    let mut damage = source.pet.stats.attack;
    let mut damage_type = DamageType::Attack;   

    match source.pet.held_food {
        Some(Food::MeatBone) => {
            damage += 3;
        },        
        Some(Food::Steak) => {
            damage += 20;
            source.pet.held_food = None;
        },
        Some(Food::Cheese) => {
            damage *= 2;
            source.pet.held_food = None;
        },
        Some(Food::FortuneCookie) => {
            if rand::thread_rng().gen_bool(0.5) {
                damage *= 2;
            }
        },        
        Some(Food::Salt) => {
            if source.pet.species.tier() > target.pet.species.tier() {
                damage *= 2;
            }
        },        
        Some(Food::Peanut) => {
            damage_type = DamageType::Instakill;
        },
        _ => {},
    }

    hurt(damage, source.pet.id, damage_type, target, action_resolver)
}

pub fn simulate_battle(mut state: PlayerState) -> GameResult {
    let mut resolver = ActionResolver::new();

    // Add start of battle abilities
    resolver.trigger_multiple(&mut state.get_all_pets(), Trigger::StartOfBattle);

    resolver.resolve(&mut state);

    // Loop through the pets attacking each other until one side has no pets left
    loop {
        // TODO: Empty front space abilities

        // Push pets to front before every attack I think
        state.push_to_front();
        // Add triggers for possible reordering 
            for pet in state.get_all_pets() {
        trigger_action(&mut resolver, &mut pet, Trigger::PossibleReordering);
    }

        let [player_pet, opposition_pet] = state.get_front_pets();

        match (player_pet, opposition_pet) {
            // No one has pets left
            (None, None) => return GameResult::Draw,

            // You have no pets left
            (None, Some(_)) => return GameResult::Loss,

            // They have no pets left
            (Some(_), None) => return GameResult::Win,

            // Both have pets left
            (Some(player_pet), Some(opposition_pet)) => {
                // Before attack abilities
                trigger_action(&mut resolver, &mut player_pet, Trigger::BeforeAttack);                
                trigger_action(&mut resolver, &mut player_pet, Trigger::BeforeAttack);

            },
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
        mut target: PetDetails, action_resolver: &mut ActionResolver) 
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
    trigger_action(action_resolver, &mut target, trigger);

    // Check if the pet has been killed
    if new_hp == 0 {
        todo!()
        // Pet is dead
    }
}



