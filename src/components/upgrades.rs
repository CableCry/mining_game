use bevy::{
    prelude::*,
};


use crate::types::SingleValue;
use crate::impl_single_value;


#[derive(Component)]
pub struct MoneyPerClick(f64);
impl_single_value!(MoneyPerClick, f64);


#[derive(Component)]
pub struct MoneyPerSecond(f64);
impl_single_value!(MoneyPerSecond, f64);


#[derive(Component)]
pub struct MoneyMultiplier(f64);
impl_single_value!(MoneyMultiplier, f64);






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
