
use bevy::{
    color::palettes::basic::*,
    input_focus::InputFocus,
    prelude::*,
};


use crate::components::inventory::*;
use crate::components::player::*;


//TODO:
// 1. Add customization on load 
//      - Text, size, etc...
// 2. Link functions like increase
// 3. Build out better system for the button bundle 
// 4. Look into belly and if it is worth adding






pub fn button(asset_server: &AssetServer, text: impl Into<String>) -> impl Bundle {
    
    let label = text.into(); 
    (
        
        // Outer container: NOT a Button
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            // Inner clickable button
            Button,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(2)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BorderRadius::new(
                Val::Px(0.0),
                Val::Px(0.0),
                Val::Px(0.0),
                Val::Px(0.0),
            ),
            BackgroundColor(Color::BLACK),
            children![(
                Text::new(label),
                TextFont {
                    font: asset_server.load("fonts/monogram.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}




const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);





#[derive(Component)]
pub enum UiAction {
    IncreaseBalance(),
    DecreaseBalance(),
    Test(String),
}


pub fn button_interaction_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        // Find the first child that actually has a Text component
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(RED);
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
            }
            Interaction::None => {
                input_focus.clear();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}


pub fn button_action_system(

    mut inventory: Single<&mut Inventory, With<Player>>,
    buttons: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<Button>)>,
    actions: Query<&UiAction>
) {

    for (interaction, child_of) in &buttons {

        if *interaction != Interaction::Pressed {
            continue;
        }

        let parent_entity = child_of.parent();

        if let Ok(action) = actions.get(parent_entity) {
            
            match action {
                UiAction::IncreaseBalance() => { inventory.add_money(1.0); }
                UiAction::DecreaseBalance() => { inventory.add_money(-1.0); }
         
                UiAction::Test(label) => {
                    println!("TEST BUTTON HIT {label}")
                }
            }
        }

    }

}
