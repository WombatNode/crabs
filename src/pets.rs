use std::ops::Index;

use crate::{states::{Id, Team, Side}, stats::Stats, battles::Perspective, triggers::Trigger, actions::ActiveAction};


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

pub fn trigger_action(mut perspective: Perspective, trigger: Trigger) {

    let pet = match perspective.get_current_pet() {
        Some(pet) => pet,
        None => return,
    };

    match trigger {
        Trigger::StartOfBattle => {
            match pet.species {
                Species::Mosquito => {
                    // Snipe modification
                    let snipe_damage = 1;
                    let attack = pet.stats.attack;
                    perspective.action_resolver.active_actions.add(ActiveAction {
                        priority: attack,
                        delayed: false,
                        action: Box::new(|| {
                            return ;
                        }),
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
