use std::cmp::{Ordering, min};

use rand::{thread_rng, Rng, rngs::ThreadRng, seq::{SliceRandom, SliceChooseIter, IteratorRandom}};

use crate::{pets::{Pet, PetDetails}, states::Team};


// In order to have more consistent randomness, and maybe to save overhead (I have no clue how stuff works behind the scenes), I may not want to initialise my threadrng everytime i use it
// pub fn cmp_random(a: Pet, b: Pet, mut rng: ThreadRng) -> Ordering {
// Random comparator
pub fn cmp_random(a: &Pet, b: &Pet) -> Ordering {
    let mut rng = thread_rng();

    if rng.gen_bool(0.5) {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

pub fn is_living(pet: &PetDetails) -> bool {
    return pet.pet.stats.hp > 0;
}

pub fn randomise(pets: &mut Vec<PetDetails>) {
    pets.shuffle(&mut rand::thread_rng())
}