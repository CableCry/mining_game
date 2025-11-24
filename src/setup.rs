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


const SHOW_BORDER: bool = false;












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
    


    let mut base = commands.spawn(
        Node {
            width: percent(100.0),
            height: percent(100.0),
            border: UiRect::all(px(2.0)),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(px(10.0)),
            ..default()
        });


    base.with_children(|parent| {
        let mut inner = parent.spawn(Node {
            width: percent(25.0),
            height: percent(35.0),
            border: UiRect::all(px(2.0)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        });

        if SHOW_BORDER { inner.insert(BorderColor::all(Color::WHITE)); }

        inner.with_children(|parent| {

            parent.spawn((
                button(&asset_server, "Increase"),
                UiAction::Balance("Increase".to_string())  
                
            ));
            parent.spawn(
                (button(&asset_server, "Test 1"), 
                UiAction::Test("Test".to_string())
            ));
            parent.spawn((
                button(&asset_server, "Test 2"),
                UiAction::Test("Test".to_string())
            ));
        });
    });

}



