use std::cell::Cell;
use std::collections::{HashSet};

use crate::pets::Pet;
use crate::food::Food;

pub struct Team {
    pets: [Option<Pet>; 5],
}

enum ShopSlot {
    None,
    Pet {
        pet: Pet,
    },
    Food {
        food: Food,
        cost: u8,
    },
}

pub struct State {
    team: Team,
    shop: [ShopSlot; 9]
}

pub struct BattlePerspective<'a> {
    team: &'a mut Team,
    opposition: &'a mut Team,
}

// Get a Vec containing the actions with highest priority values
fn get_largest_random_action<'a>(mut action_iter: impl Iterator<Item = &'a Action<'a>>) -> Vec<Action<'a>> {
    // We should never be trying to run this function on an empty ActionQueue, so throwing an error is fine 
    let first = action_iter.next().unwrap();
    let mut priority = first.priority;
    let mut priority_actions = vec!(first);

    for action in action_iter {
        match action.priority.cmp(priority) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => {
                priority_actions.push(action);
            },
            std::cmp::Ordering::Greater => {
                priority = action.priority;
                priority_actions = vec!(action);
            },
        }
    }

    priority_actions
}

// A queue for actions
// I don't think this is an accurate representation of the game's real logic
pub struct ActionQueue<'a> {
    // Whether the queue is currently in a delay state. Used for actions that are delayed
    // If delay is true, and an action's delay is true, then that action should only be chosen if there are no non-delay actions left
    delay: bool,
    // Currently there is just an unsorted array of all the actions. 
    // This has a kind of bad time complexity, as we will just be getting the largest (ish) priority items 
    buffered_actions: HashSet<Action<'a>>,
}



impl <'a> ActionQueue<'a>{
    // Return the next queues action
    fn get_next(&mut self) -> Option<Action<'a>> {
        if self.buffered_actions.is_empty(){
            return None
        }
        let action_iter = self.buffered_actions.iter();
        let max_actions = if (self.delay) {
            // Make iterator peekable in order to check if it is empty
            // Get rid of all delayed actions
            let mut delayed_actions = action_iter
                .filter(|action| !action.delaying)
                .peekable();
            if delayed_actions.peek().is_some() {
                get_largest_random_action(delayed_actions)
            } else {
                get_largest_random_action(action_iter)
            }
        } else {
            get_largest_random_action(action_iter)
        };

        todo!()
    }
}

pub struct Action<'a> {
    priority: &'a Cell<i32>,
    delaying: bool,
    action: Box<dyn FnOnce()>
}

pub enum GameResult {
    Win,
    Loss,
}

pub fn simulate_battle(state: BattlePerspective) -> GameResult {

    GameResult::Loss
}