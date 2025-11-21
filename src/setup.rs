use bevy::{
    color::palettes::css,
    prelude::*,
};

use crate::components::*;


pub fn setup_player(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
    
    commands.spawn((
        Player,
        Inventory::new(0.0, 0.0),
    ));

}


pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    commands
        .spawn((
            // Create a Text with multiple child spans.
            Text::new("Money: "),
            TextFont {
                // This font is loaded and will be used instead of the default font.
                font: asset_server.load("fonts/monogram.ttf"),
                font_size: 42.0,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            (
                TextFont {
                    font: asset_server.load("fonts/monogram.ttf"),
                    font_size: 42.0,
                    ..Default::default()
                },
                TextColor(css::GOLD.into()),
            ),
            MoneyText,
        ));
    
    commands.spawn(button(&asset_server));
}



