use crate::states::Id;



pub enum Trigger {
    StartOfBattle,
    Hurt{source: Id},
    Faint,
    Sold,

}