use crate::{states::Id, battles::DamageType};


#[derive(Clone)]
pub enum Trigger {
    StartOfBattle,
    Hurt{source: Id, damage_type: DamageType},
    Faint,
    Sold,
    PossibleReordering,
    BeforeAttack,
    AfterAttack,
}