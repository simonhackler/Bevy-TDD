use bevy::{prelude::*};
use crate::{board::*};

#[derive(Component)]
pub struct EnemyHealth {
    pub health: u32,
}

#[derive(Component)]
pub struct EnemyWalking{
    pub speed: f32,
}

#[derive(Bundle)]
struct BasicEnemy {
    health: EnemyHealth,
    walking: EnemyWalking,
}

enum EnemyTypes {
    BasicEnemy(BasicEnemy),
}

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (walk_enemies, check_enemy_health));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn check_enemy_health(
    mut commands: Commands,
    query: Query<(&EnemyHealth, Entity), Changed<EnemyHealth>>,
) {
    for (health, entity) in query.iter() {
        if health.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}