use std::cmp::{Ordering, min};

use rand::{thread_rng, Rng, rngs::ThreadRng, seq::{SliceRandom, SliceChooseIter, IteratorRandom}};

use crate::{pets::Pet, states::Team};


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

fn pick_n_random(n: usize, team: &mut Team) {

    // let some_elements: Vec<&mut Pet> = team.into_iter()
    //     .filter_map(|x| x.as_ref())
    //     .collect();
    // let n = min(some_elements.len(), n);

    // let random_elements = some_elements.into_iter()
    //     .choose_multiple(&mut rand::thread_rng(), n);


    // We shouldn't choose empty slots
    // let pets = team.into_iter()
    //     .filter_map(|pet| pet)

    // let mut team_indices: Vec<usize> = Vec::new();

    // for i in 0..5 {
    //     if team.get(i).is_some() {
    //         team_indices.push(i);
    //     }
    // }

    // let n = min(n, team_indices.len());

    // let mut rng = &mut rand::thread_rng();
    // let chosen_indices = team_indices.choose_multiple(rng, n);

    // let mut pets = Vec::new();

    // for i in chosen_indices {
    //     let t = team.get(i.clone()).unwrap();
    //     pets.push(t);

    // }   

    // let pets= chosen_indices
    //     .map(|index| team.get(index.clone()).unwrap());

    // pets

    todo!()
}   

// pub fn apply_to_n_random() {

// }