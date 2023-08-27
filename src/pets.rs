use std::ops::Index;

use crate::{states::Id, stats::Stats, battles::Perspective, triggers::Trigger};

pub struct Pet {
    pub species: Species,
    pub base_stats: Stats,
    pub temp_stats: Stats,
    pub stats: Stats,
    pub xp: u8,
    pub id: Id,
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
                },
                Species::Crab => todo!(),
                Species::Dodo => todo!(),
                Species::Dolphin => todo!(),
                Species::Skunk => todo!(),
                _ => (),
            }
        },
        Trigger::Hurt { source } => todo!(),
        Trigger::Faint => todo!(),
        Trigger::Sold => todo!(),
    }
}
