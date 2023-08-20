use std::cell::Cell;
use std::collections::{HashSet};
use std::ptr::addr_of;
use std::hash::{Hash, Hasher};

use rand::{self, Rng};

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

// Get a random action of the highest priority
fn get_largest_random_action<'a>(mut action_iter: impl Iterator<Item = &'a Action<'a>>) -> &'a Action<'a> {
    // We should never be trying to run this function on an empty ActionQueue, so throwing an error is fine 
    let first = action_iter.next().unwrap();
    let mut priority = first.priority;
    let mut priority_actions = vec!(first);

    // Add highest priority actions to vec
    for action in action_iter {
        match action.priority.cmp(priority) {
            std::cmp::Ordering::Less => (), // Low priority - ignore
            std::cmp::Ordering::Equal => {
                // Same priority as best, add to vec for random choosing
                priority_actions.push(action);
            },
            std::cmp::Ordering::Greater => {
                // Higher priority than any others; get rid of old stuff
                priority = action.priority;
                priority_actions = vec!(action);
            },
        }
    }

    // Randomly pick an action
    let n_actions = priority_actions.len();
    let chosen_index = rand::thread_rng().gen_range(0..n_actions);
    priority_actions.get(chosen_index).unwrap()
}

// A queue for actions
// I don't think this is an accurate representation of the game's real logic
pub struct ActionQueue<'a> {
    // Whether the queue is currently in a delay state. Used for actions that are delayed
    // If delay is true, and an action's delay is true, then that action should only be chosen if there are no non-delay actions left
    // This delay state idea is based on Freetz's comments in the discord, IDK if they are correct, or if I correctly interpreted their ideas
    delay: bool,
    // Currently there is just an unsorted array of all the actions. 
    // This has a kind of bad time complexity, as we will just be getting the largest (ish) priority items 
    buffered_actions: HashSet<Action<'a>>,
}



impl <'a> ActionQueue<'a>{
    // Return the next queues action
    fn get_next(&mut self) -> Option<Action<'a>> {
        // Return None if there are no buffered actions
        if self.buffered_actions.is_empty(){
            return None
        }

        // There are buffered actions
        // Choose one
        let action_iter = self.buffered_actions.iter();
        // If the delay flag is set in the queue, that means we don't want to choose delayed actions
        let max_action = if self.delay {
            // Make iterator peekable in order to check if it is empty
            // Get rid of all delayed actions
            let mut delayed_actions = action_iter
                .filter(|action| !action.delayed)
                .peekable();
            if delayed_actions.peek().is_some() {
                get_largest_random_action(delayed_actions)
            } else {
                get_largest_random_action(action_iter)
            }
        } else {
            get_largest_random_action(action_iter)
        };

        // Take chosen action out of the action buffer
        let action: Action = self.buffered_actions.take(max_action).unwrap();

        todo!()
    }
}

pub struct Action<'a> {
    priority: &'a Cell<i32>,
    delayed: bool,
    action: Box<dyn FnOnce()>
}


// I am using addr_of for all my equality checks and whatnot
// This might be a bad idea, though I'm not exactly sure why
impl PartialEq for Action<'_> {
    fn eq(&self, other: &Self) -> bool {
        addr_of!(self) == addr_of!(other)
    }
}
impl Eq for Action<'_> {}

impl Hash for Action<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
            
    }
}

pub enum GameResult {
    Win,
    Loss,
}

pub fn simulate_battle(state: BattlePerspective) -> GameResult {

    GameResult::Loss
}