mod components;
mod setup;
mod update;


use bevy::{
    input_focus::InputFocus,
    prelude::*
};


// TODO: Need to work on the Ui, add buttons and actions for the functions


fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.init_resource::<InputFocus>();
    app.add_systems(Startup, (setup::setup_player, setup::setup_ui));
    app.add_systems(Update, (update::update, components::button_system));

    app.run();
}


//fn draw (
//    query: Query<&Inventory, With<Player>>,
//){
//
//}

