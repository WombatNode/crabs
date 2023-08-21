use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::Pet};



pub fn simulate_battle(state: PlayerState) -> GameResult {

    GameResult::Loss
}

// A wrapper around gamestate to be used in battle
// provides generic ways to interact with the game's state
pub trait BattlePerspective<'a> {
    fn get_side(&self) -> Side;
    // If provided, represents which position in the lineup the pet doing the action is in
    fn get_state(&mut self) -> &'a mut PlayerState;
}

// BattlePerspective without a pet as a focus
pub struct BasicPerspective<'a> {
    state: &'a mut PlayerState,
    side: Side,
}

// BattlePerspective which contains a pet as a focus, which allows for more functionality
pub struct PetPerspective<'a> {
    state: &'a mut PlayerState,
    side: Side,
    pet_position: Option<usize>,
}

impl <'a> BasicPerspective<'a> {
    // Create a new BattlePerspective based on the side of the battle that we are currently on
    fn new(state: &'a mut PlayerState, side: Side) -> BasicPerspective<'a> {
        BasicPerspective { 
            state, 
            side, 
        }
    }    
}

fn get_pet_index_from_id(team: &mut Team, id: Id) -> Option<usize> {
    team.into_iter()
        .enumerate()
        .find_map(|(index, pet)| {
            match pet {
                Some(pet) if pet.id == id => {
                    Some(index)
                },
                _ => None,
            }
        })
} 

impl <'a> PetPerspective<'a> {
    // Create a new BattlePerspective based on the side of the battle that we are currently on
    fn new(state: &'a mut PlayerState, side: Side, id: Id) -> PetPerspective<'a> {
        // We shouldn't be trying to find a pet from the opposing team while in the shop
        let team = state.get_team_from_side(side).unwrap();
        // Find the pet
        let pet_position = get_pet_index_from_id(team, id);
        
        PetPerspective { 
            state, 
            side,
            pet_position, 
        }
    }    
}

