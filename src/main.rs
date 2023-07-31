use bevy::{prelude::*, render::camera::{ScalingMode}, ecs::query};

mod towers;
mod board;
mod ui;
mod enemies;

use board::*;


#[derive(Component)]
pub struct MainCamera;


fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "PvZ Rougelike".into(),
                        resolution: (1920.0, 1080.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins((towers::TowerPlugin, enemies::EnemiesPlugin))
        .insert_resource(towers::Money(1000))
        .insert_resource(board::generate_board())
        .add_systems(Startup, (setup, ui::spawn_ui))
        .add_systems(Update, (ui::update_money, ))
        .add_systems(Update, gizmos_grid)
        .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1280.0,
        min_height: 720.0,
    };

    commands.spawn((camera, MainCamera));
}


fn gizmos_grid(mut gizmos: Gizmos) {
    for i in -10..10 {
        gizmos.line_2d(Vec2::new(GRID_X_SPACING * (i as f32), -500.0), Vec2::new(GRID_X_SPACING * (i as f32), 500.), Color::RED);
    }
    for i in -4..4 {
        gizmos.line_2d(Vec2::new(-600.0, GRID_Y_SPACING * (i as f32)), Vec2::new(600.0, GRID_Y_SPACING * (i as f32)), Color::RED);
    }
}
