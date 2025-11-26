use bevy::prelude::*;

use crate::components::*;


pub fn update(
    // query: Query<(&Player, &mut Inventory)>,
    inventory: Single<&mut Inventory, With<Player>>,
    mut money_text: Single<&mut TextSpan, With<MoneyText>>,
) {
    let money = inventory.money();
    **money_text = format!("{money:.2}").into();
    //let span: &mut TextSpan = &mut *money_text;
    //span.0 = format!("{money:.2}");

}


