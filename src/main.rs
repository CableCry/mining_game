mod components;
mod setup;
mod update;

use bevy::prelude::*;





fn main() {

    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    app.add_systems(Startup, (setup::setup_player, setup::setup_ui));
    app.add_systems(Update, update::update);

    app.run();
}


//fn draw (
//    query: Query<&Inventory, With<Player>>,
//){
//
//}

