
use bevy::{
    color::palettes::basic::*,
    input_focus::InputFocus,
    prelude::*,
};



pub fn button(asset_server: &AssetServer) -> impl Bundle {
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
                border: UiRect::all(px(5)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BorderRadius::MAX,
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Button"),
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



pub fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (entity, interaction, mut color, mut border_color, children) in &mut interaction_query {
        // Find the first child that actually has a Text component
        if let Some(&child) = children.first() {
            if let Ok(mut text) = text_query.get_mut(child) {
                match *interaction {
                    Interaction::Pressed => {
                        input_focus.set(entity);
                        text.0 = "Press".to_string();
                        *color = PRESSED_BUTTON.into();
                        *border_color = BorderColor::all(RED);
                    }
                    Interaction::Hovered => {
                        input_focus.set(entity);
                        text.0 = "Hover".to_string();
                        *color = HOVERED_BUTTON.into();
                        *border_color = BorderColor::all(Color::WHITE);
                    }
                    Interaction::None => {
                        input_focus.clear();
                        text.0 = "Button".to_string();
                        *color = NORMAL_BUTTON.into();
                        *border_color = BorderColor::all(Color::BLACK);
                    }
                }
            }
        }
    }
}

