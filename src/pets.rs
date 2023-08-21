use std::ops::{Add, AddAssign};

use crate::states::Id;

#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
}

impl Add for Stats {
    type Output = Stats;

    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            hp: self.hp + rhs.hp,
            attack: self.attack + rhs.attack,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.attack += rhs.attack;
    }
}

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

