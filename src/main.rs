use bevy::{prelude::*, render::camera::{ScalingMode}};

mod towers;
mod board;
mod ui;

use board::*;


#[derive(Component)]
pub struct MainCamera;


#[derive(Component)]
pub struct EnemyHealth {
    pub health: u32,
}

#[derive(Component)]
pub struct EnemyWalking{
    pub speed: f32,
}

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
        .insert_resource(towers::Money(100))
        .insert_resource(towers::SelectableTowers {
            possible_towers: towers::setup_tower_costs(),
            selected_tower: None,
        })
        .insert_resource(board::generate_board())
        .add_event::<towers::MoneyUpdated>()
        .add_systems(Startup, (setup, ui::spawn_ui))
        .add_systems(Update, (walk_enemies, towers::spawn_tower_at_mouse, towers::update_money, towers::select_tower, ui::update_money))
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

    spawn_enemy(&mut commands, &asset_server, 0.0);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING * 2.0);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING * -1.0);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING * -2.0);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING * -3.0);
    spawn_enemy(&mut commands, &asset_server, GRID_Y_SPACING * -4.0);
}

fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, y: f32) {
    let texture = asset_server.load("kenney/PNG/DefaultSize/towerDefense_tile247.png");
    commands.spawn((SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(500.0, y + GRID_Y_SPACING / 2.0, 0.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale: Vec3::new(1.0, -1.0, 1.0),
        },
        ..default()
    }, EnemyHealth { health: 100 }, EnemyWalking { speed: 10.0 })
    );
}

fn walk_enemies(
    mut enemies: Query<(&mut Transform, &EnemyWalking)>,
    time: Res<Time>
) {
    for (mut transform, enemy) in enemies.iter_mut() {
        transform.translation.x -= enemy.speed * time.delta_seconds();
    }
}



fn gizmos_grid(mut gizmos: Gizmos) {
    for i in -10..10 {
        gizmos.line_2d(Vec2::new(GRID_X_SPACING * (i as f32), -500.0), Vec2::new(GRID_X_SPACING * (i as f32), 500.), Color::RED);
    }
    for i in -4..4 {
        gizmos.line_2d(Vec2::new(-600.0, GRID_Y_SPACING * (i as f32)), Vec2::new(600.0, GRID_Y_SPACING * (i as f32)), Color::RED);
    }
}
