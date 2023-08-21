use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Default, Clone, Copy)]
pub struct Stats {
    pub hp: u8,
    pub attack: u8,
}

impl Add for Stats {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            hp: self.hp + rhs.hp,
            attack: self.attack + rhs.attack,
        }
    }
}

impl AddAssign for Stats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.attack += rhs.attack;
    }
}

impl Sub for Stats {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Stats {
            hp: self.hp - rhs.hp,
            attack: self.attack - rhs.attack,
        }
    }
}

impl SubAssign for Stats {
    fn sub_assign(&mut self, rhs: Self) {
        self.hp -= rhs.hp;
        self.attack -= rhs.attack;
    }
}