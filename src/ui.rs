use bevy::prelude::*;

use crate::towers::MoneyUpdated;

#[derive(Component)]
pub struct MoneyText;

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Money: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::GOLD,
            }),
        ]),
        MoneyText,
    ));
}

pub fn update_money(
    mut money_updated: EventReader<MoneyUpdated>,
    mut query: Query<&mut Text, With<MoneyText>>,
) {
    for value in money_updated.iter() {
        for mut text in &mut query {
            let val = value.new_value;
            text.sections[1].value = format!("{val}");
        }
    }
}