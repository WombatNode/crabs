use crate::{states::Id, battles::DamageType};



pub enum Trigger {
    StartOfBattle,
    Hurt{source: Id, damage_type: DamageType},
    Faint,
    Sold,

}