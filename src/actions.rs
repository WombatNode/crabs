use std::cell::Cell;

use rand::{self, Rng};

type ActionFn = dyn FnOnce();

// Get the index of a random action of the highest priority when given an enumerated iterator
fn get_largest_random_action<'a, T>(mut action_iter: impl Iterator<Item = (usize, &'a T)>) -> usize
where
    T: Action + 'a
{
    // We should never be trying to run this function on an empty ActionQueue, so throwing an error is fine 
    let (first_index, first) = action_iter.next().unwrap();
    let mut priority = first.get_priority();
    let mut priority_indices = vec!(first_index);

    // Add highest priority actions to vec
    for (index, action) in action_iter {
        match action.get_priority().cmp(&priority) {
            std::cmp::Ordering::Less => (), // Low priority - ignore
            std::cmp::Ordering::Equal => {
                // Same priority as best, add to vec for random choosing
                priority_indices.push(index);
            },
            std::cmp::Ordering::Greater => {
                // Higher priority than any others; get rid of old stuff
                priority = action.get_priority();
                priority_indices = vec!(index);
            },
        }
    }

    // Randomly pick an action
    let n_actions = priority_indices.len();
    let chosen_index = rand::thread_rng().gen_range(0..n_actions);
    *priority_indices.get(chosen_index).unwrap()
}


// A queue for actions
// I don't think this is an accurate representation of the game's real logic
pub struct ActionQueue<T: Action> {
    // Whether the queue is currently in a delay state. Used for actions that are delayed
    // If delay is true, and an action's delay is true, then that action should only be chosen if there are no non-delay actions left
    // This delay state idea is based on Freetz's comments in the discord, IDK if they are correct, or if I correctly interpreted their ideas
    delay: bool,
    // Currently there is just an unsorted array of all the actions. 
    // This has a kind of bad time complexity, as we will just be getting the largest (ish) priority items 
    buffered_actions: Vec<T>,
}



impl <T: Action> ActionQueue<T>{
    // Return the next queues action
    pub fn get_next(&mut self) -> Option<T> {
        // Return None if there are no buffered actions
        if self.buffered_actions.is_empty(){
            return None
        }

        // There are buffered actions
        // Choose one
        let action_iter = self.buffered_actions.iter().enumerate();
        // If the delay flag is set in the queue, that means we don't want to choose delayed actions
        let max_action = if self.delay {
            // Make iterator peekable in order to check if it is empty
            // Get rid of all delayed actions
            let mut delayed_actions = action_iter
                .filter(|(_, action)| !action.is_delayed())
                .peekable();
            if delayed_actions.peek().is_some() {
                get_largest_random_action(delayed_actions)
            } else {
                // All actions are delayed. We will need to recalculate the list of actions
                let action_iter = self.buffered_actions.iter().enumerate();
                get_largest_random_action(action_iter)
            }
        } else {
            get_largest_random_action(action_iter)
        };

        // Take chosen action out of the action buffer
        let action: T = self.buffered_actions.swap_remove(max_action);

        Some(action)
    }
}

// Action for potentially dead pets - priority doesn't change
pub struct FinalAction {
    priority: i32,
    delayed: bool,
    action: Box<ActionFn>
}
// Action for living pets - priority automatically changes on attack change
pub struct ActiveAction<'a> {
    priority: &'a Cell<i32>,
    delayed: bool,
    action: Box<ActionFn>
}

pub trait Action {
    fn get_priority(&self) -> i32;
    fn is_delayed(&self) -> bool;
    fn take_action(self) -> Box<ActionFn>;
}

impl Action for FinalAction {
    fn get_priority(&self) -> i32 {
        self.priority
    }

    fn is_delayed(&self) -> bool {
        self.delayed
    }

    fn take_action(self) -> Box<ActionFn> {
        self.action
    }
}

impl <'a> Action for ActiveAction<'a> {
    fn get_priority(&self) -> i32 {
        self.priority.get()
    }

    fn is_delayed(&self) -> bool {
        self.delayed
    }

    fn take_action(self) -> Box<ActionFn> {
        self.action
    }
}