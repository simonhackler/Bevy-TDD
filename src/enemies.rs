use lazy_static::lazy_static;
use rand::Rng;
use bevy::{prelude::*, utils::HashMap, utils::Duration};
use crate::{board::*, towers};

#[derive(Component, Clone, Copy)]
pub struct EnemyHealth {
    pub health: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Walking{
    pub speed: f32,
}

#[derive(Component, Clone, Copy)]
pub struct Damage{
    pub damage: i32,
    pub colliding: bool
}

#[derive(Bundle, Clone)]
struct BasicEnemy {
    health: EnemyHealth,
    walking: Walking,
    damage: Damage,
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Enemies{
    Basic,
    BasicHighHealth,
    FastBasic,
}

#[derive(Component)]
pub struct SpawnTimer {
    pub timer: Timer,
    pub index: usize,
}

lazy_static! {
    static ref ENEMY_IMPLEMENTATIONS: HashMap<Enemies, (BasicEnemy, String)> = {
        let mut map = HashMap::new();
        map.insert(Enemies::Basic, (BasicEnemy {
            health: EnemyHealth { health: 100 },
            walking: Walking { speed: 15.0 },
            damage: Damage { damage: 10, colliding: false },
        }, "kenney/PNG/DefaultSize/towerDefense_tile247.png".to_string()));
        map.insert(Enemies::BasicHighHealth, (BasicEnemy {
            health: EnemyHealth { health: 300 },
            walking: Walking { speed: 15.0 },
            damage: Damage { damage: 10, colliding: false },
        }, "kenney/PNG/DefaultSize/towerDefense_tile246.png".to_string()));
        map.insert(Enemies::FastBasic, (BasicEnemy {
            health: EnemyHealth { health: 100 },
            walking: Walking { speed: 35.0 },
            damage: Damage { damage: 10, colliding: false },
        }, "kenney/PNG/DefaultSize/towerDefense_tile248.png".to_string()));
        map
    };
}

const TEST_WAVE: [(Enemies, f32); 10] = [
    (Enemies::Basic, 0.0),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::Basic, 0.1),
    (Enemies::BasicHighHealth, 0.1),
    (Enemies::FastBasic, 0.1),
];


const WAVE_1: [(Enemies, f32); 22] = [
    (Enemies::Basic, 45.0),
    (Enemies::Basic, 25.0),
    (Enemies::Basic, 15.0),
    (Enemies::Basic, 10.0),
    (Enemies::Basic, 5.0),
    (Enemies::BasicHighHealth, 20.0),
    (Enemies::FastBasic, 15.0),
    (Enemies::BasicHighHealth, 15.0),
    (Enemies::Basic, 5.0),
    (Enemies::Basic, 0.3),
    (Enemies::Basic, 0.3),
    (Enemies::Basic, 0.3),
    (Enemies::BasicHighHealth, 20.0),
    (Enemies::FastBasic, 3.0),
    (Enemies::Basic, 0.0),
    (Enemies::Basic, 0.0),
    (Enemies::Basic, 0.0),
    (Enemies::Basic, 0.0),
    (Enemies::BasicHighHealth, 0.0),
    (Enemies::BasicHighHealth, 0.0),
    (Enemies::FastBasic, 0.0),
    (Enemies::FastBasic, 0.0),
];

const SPAWN_POSITIONS: [f32; 7] = [
    GRID_Y_SPACING * -1.0,
    GRID_Y_SPACING * -2.0,
    GRID_Y_SPACING * -3.0,
    GRID_Y_SPACING * -4.0,
    0.0,
    GRID_Y_SPACING,
    GRID_Y_SPACING * 2.0,
];


pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (walk_enemies, check_enemy_health, update_spawn_timer, enemies_damage_towers));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(SpawnTimer {
        timer: Timer::from_seconds(WAVE_1[0].1, TimerMode::Repeating),
        index: 0,
    });
}

fn update_spawn_timer(
    time: Res<Time>, mut spawn_timer: Query<&mut SpawnTimer>,
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    for mut timer in spawn_timer.iter_mut() {
        timer.timer.tick(time.delta());                                                 
        if timer.timer.finished() {
            if timer.index >= WAVE_1.len() {
                return;
            } else {
                let enemy_timing = &WAVE_1[timer.index];
                timer.timer.set_duration(Duration::from_secs_f32(enemy_timing.1));
                println!("Timer: {:?}", timer.timer.duration());
                let random_index = rand::thread_rng().gen_range(0..=6);
                let enemy = &ENEMY_IMPLEMENTATIONS[&enemy_timing.0];
                let texture: Handle<Image> = asset_server.load(&enemy.1);
                spawn_enemy(&mut commands, texture, SPAWN_POSITIONS[random_index], enemy.0.clone());
            }
            timer.index += 1;
        }
    }
}

fn spawn_enemy(commands: &mut Commands, texture: Handle<Image>, y: f32, enemy: BasicEnemy) {
    commands.spawn((SpriteBundle {
        texture,
        transform: Transform {
            translation: Vec3::new(500.0, y + GRID_Y_SPACING / 2.0, 0.0),
            rotation: Quat::from_rotation_y(std::f32::consts::PI),
            scale: Vec3::new(1.0, -1.0, 1.0),
        },
        ..default()
    }, enemy)
    );
}

fn walk_enemies(
    mut enemies: Query<(&mut Transform, &Walking, &Damage)>,
    time: Res<Time>
) {
    for (mut transform, enemy, enemy_damage) in enemies.iter_mut() {
        if !enemy_damage.colliding {
            transform.translation.x -= enemy.speed * time.delta_seconds();
        }
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

fn enemies_damage_towers (
    mut commands: Commands,
    mut enemies: Query<(&Transform, &mut Damage)>,
    mut towers: Query<(&Transform, &mut towers::Health)>,
) {
    for (transform, mut enemy) in enemies.iter_mut() {
        enemy.colliding = false;
        for (tower_transform, mut tower ) in &mut towers {
            if transform.translation.y == tower_transform.translation.y && (transform.translation.x - tower_transform.translation.x).abs() < 30.0 {
                tower.health -= enemy.damage;
                enemy.colliding = true;
            }
        }
    } 
}