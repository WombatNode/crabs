use std::ops::Index;

use crate::{states::{Id, Team, Side, PlayerState}, stats::Stats, triggers::Trigger, actions::{Action, ActionResolver}};


pub struct Pet {
    pub species: Species,
    pub base_stats: Stats,
    pub temp_stats: Stats,
    pub stats: Stats,
    pub xp: u8,
    pub id: Id,
}

pub struct PetDetails<'a> {
    pet: &'a mut Pet,
    side: Side,
    position: usize,
}

pub enum Species {
    Duck,
    Beaver,
    Otter,
    Pig,
    Ant,
    Mosquito,
    Mouse,
    Fish,
    Cricket,
    Horse,
    Snail,
    Crab,
    Swan,
    Rat,
    Hedgehog,
    Peacock,
    Flamingo,
    Worm,
    Kangaroo,
    Spider,
    Dodo,
    Badger,
    Dolphin,
    Zebra,
    Elephant,
    Camel,
    Rabbit,
    Ox,
    Dog,
    Sheep,
    Skunk,
    Hippo,
    Bison,
    Blowfish,
}

impl Pet {
    pub fn apply_trigger(&mut self) {

    }
}

pub fn trigger_action(mut action_resolver: ActionResolver, pet_details: PetDetails, trigger: Trigger) {

    let pet =  pet_details.pet;
    match trigger {
        Trigger::StartOfBattle => {
            match pet.species {
                Species::Mosquito => {
                    // Snipe modification
                    let snipe_damage = 1;
                    let attack = pet.stats.attack;
                    action_resolver.active_actions.add(Action {
                        priority: attack,
                        delayed: false,
                        action: Box::new(|| {
                            return ;
                        }),
                        source: pet.id,
                    })
                },
                Species::Crab => todo!(),
                Species::Dodo => todo!(),
                Species::Dolphin => todo!(),
                Species::Skunk => todo!(),
                _ => (),
            }
        },
        Trigger::Hurt { source, damage_type } => todo!(),
        Trigger::Faint => todo!(),
        Trigger::Sold => todo!(),
    }
}
