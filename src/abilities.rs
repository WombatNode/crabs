use crate::pets::Pet;

pub trait Sell {
    fn sell(&self);
}

pub trait TrySell {
    fn try_sell(&self);
}

impl <Species> TrySell for Pet<Species> {
    fn try_sell(&self) {
        return
    }
}

impl <Species> TrySell for Pet<Species> 
where
    Species: Sell
{
    
}