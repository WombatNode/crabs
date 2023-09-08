use std::ops::Index;

use crate::{states::{Id, Team, Side, PlayerState}, stats::Stats, triggers::Trigger, actions::{Action, ActionResolver}, utils::{is_living, randomise}, battles::{hurt, DamageType}, food::Food};

pub struct Pet {
    pub species: Species,
    pub base_stats: Stats,
    pub temp_stats: Stats,
    pub stats: Stats,
    pub level: u8,
    pub xp: u8,
    pub id: Id,
    pub held_food: Option<Food>,
}

pub struct PetDetails<'a> {
    pub pet: &'a mut Pet,
    pub side: Side,
    pub position: usize,
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

impl Species {
    pub fn tier(&self) -> u8 {
        // Todo actually add data
        1
    }
}

impl Pet {
    pub fn apply_trigger(&mut self) {

    }
}

pub fn trigger_action(mut action_resolver: &mut ActionResolver, pet_details: &mut PetDetails, trigger: Trigger) {

    let pet =  pet_details.pet;
    let id = pet.id;
    match trigger {
        Trigger::StartOfBattle => {
            match pet.species {
                Species::Mosquito => {
                    // Snipe modification
                    let snipe_damage = 1;
                    let attack = pet.stats.attack;
                    let opposition = pet_details.side.opposition();
                    action_resolver.active_actions.add(Action {
                        priority: attack,
                        delayed: false,
                        action: Box::new(move |resolver: &mut ActionResolver, state: &mut PlayerState| {
                            // let state = &mut action_resolver.state;
                            let level= {
                                let source = state.get_pet(id).unwrap();
                                source.pet.level
                            };

                            // let level = source.pet.level;

                            for target in state.get_n_pets(level.into(), opposition, is_living, randomise) {
                                hurt(snipe_damage, id, DamageType::Snipe, target, resolver);
                            }

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
        Trigger::PossibleReordering => todo!(),
        Trigger::BeforeAttack => todo!(),
        Trigger::AfterAttack => todo!(),
    }
}
