use rand::{self, Rng};

use crate::{states::{PlayerState, Id}, pets::{PetDetails, trigger_action}, triggers::Trigger};

type ActionFn = dyn FnOnce(&mut ActionResolver, &mut PlayerState);

// Get the index of a random action of the highest priority when given an enumerated iterator
fn get_largest_random_action<'a>(mut action_iter: impl Iterator<Item = (usize, &'a Action)>) -> usize
{
    // We should never be trying to run this function on an empty ActionQueue, so throwing an error is fine 
    let (first_index, first) = action_iter.next().unwrap();
    let mut priority = first.priority;
    let mut priority_indices = vec!(first_index);

    // Add highest priority actions to vec
    for (index, action) in action_iter {
        match action.priority.cmp(&priority) {
            std::cmp::Ordering::Less => (), // Low priority - ignore
            std::cmp::Ordering::Equal => {
                // Same priority as best, add to vec for random choosing
                priority_indices.push(index);
            },
            std::cmp::Ordering::Greater => {
                // Higher priority than any others; get rid of old stuff
                priority = action.priority;
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
pub struct ActionQueue {
    // Whether the queue is currently in a delay state. Used for actions that are delayed
    // If delay is true, and an action's delay is true, then that action should only be chosen if there are no non-delay actions left
    // This delay state idea is based on Freetz's comments in the discord, IDK if they are correct, or if I correctly interpreted their ideas
    delay: bool,
    // Currently there is just an unsorted array of all the actions. 
    // This has a kind of bad time complexity, as we will just be getting the largest (ish) priority items 
    buffered_actions: Vec<Action>,
}



impl ActionQueue{
    pub fn new() -> Self {
        ActionQueue { 
            delay: false, 
            buffered_actions: Vec::new() 
        }
    }

    // Return the next queues action
    pub fn get_next(&mut self) -> Option<Action> {
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
                .filter(|(_, action)| !action.delayed)
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
        let action = self.buffered_actions.swap_remove(max_action);

        Some(action)
    }

    pub fn add(&mut self, action: Action) {
        self.buffered_actions.push(action);
    }

    pub fn update_priorities(&mut self, source: Id, priority: u8) {
        for action in &mut self.buffered_actions {
            if action.source == source {
                action.priority = priority;
            }
        }
    }
}

// Action for  pets
pub struct Action {
    pub priority: u8,
    pub source: Id,
    pub delayed: bool,
    pub action: Box<ActionFn>
}

pub struct ActionResolver {
    pub active_actions: ActionQueue, 
    pub faints: ActionQueue, 
    pub post_faint: ActionQueue,
}

impl ActionResolver {
    pub fn new() -> Self {
        Self {
            active_actions: ActionQueue::new(), 
            faints: ActionQueue::new(), 
            post_faint: ActionQueue::new(), 
        }
    }

    pub fn update_priorities(&mut self, source: Id, priority: u8) {
        self.active_actions.update_priorities(source, priority);

        // IDK if there is any case where these things would occur. Why would a dead pets attack change?
        self.faints.update_priorities(source, priority);
        self.post_faint.update_priorities(source, priority);
    }

    pub fn get_next(&mut self) -> Option<Action> {
        self.active_actions.get_next()
            .or(self.faints.get_next())
            .or(self.post_faint.get_next())
    }

    pub fn resolve(&mut self, state: &mut PlayerState) {
        while let Some(action) = self.get_next() {
            (action.action)(self, state);
        }
    }

    pub fn trigger_multiple<'a>(&mut self, pets: impl IntoIterator<Item=&'a mut PetDetails<'a>>, trigger: Trigger) {
        for pet in pets {
            trigger_action(self, pet, trigger.clone());
        }
    }
}
