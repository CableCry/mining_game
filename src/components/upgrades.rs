use bevy::{
    prelude::*,
};

use crate::types::*;



#[derive(Component)]
pub struct MoneyPerClick(f64);

impl ValueAccess for MoneyPerClick {
    fn get_value(&self) -> f64 {
        self.0
    }
    fn set_value(&mut self, v: f64) {
        self.0 = v;
    }
}

impl NewForSingleValue<f64> for MoneyPerClick {
    fn new(v: f64) -> Self {
        Self { 0: v }
    }
}


#[derive(Component)]
pub struct MoneyPerSecond(f64);

impl ValueAccess for MoneyPerSecond {
    fn get_value(&self) -> f64 {
        self.0
    }
    fn set_value(&mut self, v: f64) {
        self.0 = v;
    }
}

impl NewForSingleValue<f64> for MoneyPerSecond {
    fn new(v: f64) -> Self {
        Self { 0: v }
    }
}



#[derive(Component)]
pub struct MoneyMultiplier(f64);

impl ValueAccess for MoneyMultiplier {
    fn get_value(&self) -> f64 {
        self.0
    }
    fn set_value(&mut self, v: f64) {
        self.0 = v;
    }
}

impl NewForSingleValue<f64> for MoneyMultiplier {
    fn new(v: f64) -> Self {
        Self { 0: v }
    }
}


enum UpgradeEffect {
    AddFlatMoneyPerClick(f64),
    AddFlatMoneyPerSecond(f64),
    MultiplyMoneyGain(f64),
}

struct UpgradeDef {
    id: &'static str,
    name: &'static str,
    cost: f64,
    effect: UpgradeEffect,
}


const UPGRADES: &[UpgradeDef] = &[
    UpgradeDef {
        id: "money_click_1",
        name: "Stronger Arms",
        cost: 10.0,
        effect: UpgradeEffect::AddFlatMoneyPerClick(1.0),
    },
];
