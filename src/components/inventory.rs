use bevy::prelude::*;



#[derive(Component)]
pub struct Inventory {
    money: f64,
    ores: f64,
}

impl Inventory {
    pub fn new(money: f64, ores: f64) -> Self {
        Self { money, ores }
    }

    pub fn money(&self) -> f64 {
        self.money
    }

    pub fn ores(&self) -> f64 {
        self.ores
    }

    pub fn add_money(&mut self, ammount: f64) {
        self.money += ammount;
    }
}





#[derive(Component)]
pub struct MoneyText;

