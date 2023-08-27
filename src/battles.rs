use crate::{states::{PlayerState, GameResult, Side, Id, Team}, pets::Pet, actions::{ActionQueue, ActiveAction, FinalAction, ActionResolver}};



pub fn simulate_battle(mut state: PlayerState) -> GameResult {
    let perspective = Perspective::new(&mut state);

    

    GameResult::Loss
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

pub fn get_pet_from_team(team: Option<&mut Team>, id: Id) -> Option<&mut Pet> {
    team?.into_iter()
        .find_map(|pet| {
            match pet {
                Some(pet) if pet.id == id => {
                    Some(pet)
                },
                _ => None,
            }
        })
} 

// A wrapper around gamestate to be used in battle, or whenever actions need to be resolved in some order
// provides generic ways to interact with the game's state
pub struct Perspective<'player, 'active> {
    pub state: &'player mut PlayerState,
    pub side: Side,
    // If provided, represents which position in the lineup the pet doing the action is in
    pub pet_id: Option<Id>,
    pub action_resolver: ActionResolver<'active>,

}

impl <'player, 'active> Perspective<'player, 'active> {
    // Create a new Perspective based on the side of the battle that we are currently on
    fn new(state: &'player mut PlayerState) -> 
        Perspective<'player, 'active> 
    {
        Perspective { 
            state, 
            side: Side::A,
            pet_id: None,
            action_resolver: ActionResolver::new(),
        }
    }

    // fn find_pet(&mut self, pet_id: Id) {
    //     // We shouldn't be trying to find a pet from the opposing team while in the shop
    //     let team = self.state.get_team_from_side(self.side).unwrap();
    //     // Find the pet
    //     self.pet_position = get_pet_index_from_id(team, pet_id);        
    // }

    pub fn get_current_pet(&mut self) -> Option<&mut Pet> {
        let pet_id = self.pet_id?;
        self.state.get_pet(pet_id)
    }

    pub fn apply_to_n_foes<F>(&mut self, f: F) 
    where 
        F: FnMut(Pet)
    {
        
    }
}

fn hurt() {
    
}








// // BattlePerspective without a pet as a focus
// pub struct BasicPerspective<'a> {
//     state: &'a mut PlayerState,
//     side: Side,
// }

// // BattlePerspective which contains a pet as a focus, which allows for more functionality
// pub struct PetPerspective<'a> {
//     state: &'a mut PlayerState,
//     side: Side,
//     // If provided, represents which position in the lineup the pet doing the action is in
//     pet_position: Option<usize>,
// }


// impl <'a> BasicPerspective<'a> {
//     // Create a new BattlePerspective based on the side of the battle that we are currently on
//     fn new(state: &'a mut PlayerState, side: Side) -> BasicPerspective<'a> {
//         BasicPerspective { 
//             state, 
//             side, 
//         }
//     }    
// }


// pub struct BasicPerspective<'a> {
//     state: &'a mut PlayerState,
//     side: Side,
// }

// // BattlePerspective which contains a pet as a focus, which allows for more functionality
// pub struct PetPerspective<'a> {
//     state: &'a mut PlayerState,
//     side: Side,
//     // If provided, represents which position in the lineup the pet doing the action is in
//     pet_position: Option<usize>,
// }
// impl <'a> PetPerspective<'a> {
//     // Create a new BattlePerspective based on the side of the battle that we are currently on
//     fn new(state: &'a mut PlayerState, side: Side, id: Id) -> PetPerspective<'a> {
//         // We shouldn't be trying to find a pet from the opposing team while in the shop
//         let team = state.get_team_from_side(side).unwrap();
//         // Find the pet
//         let pet_position = get_pet_index_from_id(team, id);
        
//         PetPerspective { 
//             state, 
//             side,
//             pet_position, 
//         }
//     }    
// }

