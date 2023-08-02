use lazy_static::lazy_static;
use bevy::{prelude::*, window::PrimaryWindow, utils::HashMap, };

use crate::{MainCamera, board::*, enemies::EnemyHealth};


#[derive(Resource)]
pub struct Money(pub u32);

#[derive(Resource)]
pub struct SelectableTowers {
    pub possible_towers: Vec<BuyableTower>,
    pub selected_tower: Option<BuyableTower>,
}

#[derive(Event)]
pub struct MoneyUpdated {
    pub new_value: u32,
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Health {
    pub health: i32,
}

#[derive(Component, Clone, Debug)]
pub struct MoneyGain {
    pub money: u32,
    pub gaintimer: Timer,
}

#[derive(Component, Clone, Debug)]
pub struct ProjectileTower {
    pub speed: f32,
    pub damage: i32,
    pub shoot_timer: Timer,
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub damage: i32,
}

#[derive(Bundle, Clone, Debug)]
pub struct MoneyBundle {
    money_tower: MoneyGain,
    health: Health,
}

#[derive(Bundle, Clone, Debug)]
pub struct SimpleProjectileBundle{
    projectile_tower: ProjectileTower,
    health: Health,
}

#[derive(Clone, Debug)]
pub enum TowerBundle {
    Money(MoneyBundle),
    SimpleProjectile(SimpleProjectileBundle)
}

#[derive(Hash,PartialEq, Eq, Debug, Clone, Copy)]
pub enum Tower {
    Money,
    NormalProjectile,
}

#[derive(Clone)]
pub struct BuyableTower {
    pub cost: u32,
    pub tower_type: Tower,
}

lazy_static! {
    static ref TOWER_IMPLEMENTATIONS: HashMap<Tower, (TowerBundle, String)> = {
        let mut map = HashMap::new();
        map.insert(
            Tower::Money, 
            (TowerBundle::Money( MoneyBundle {
                money_tower: MoneyGain {
                    money: 50, 
                    gaintimer: Timer::from_seconds(15.0, TimerMode::Repeating),
                },
                health: Health { health: 50 },
                }
            ), "art/pig.png".to_string())
        );
        map.insert(
            Tower::NormalProjectile,
            (TowerBundle::SimpleProjectile( SimpleProjectileBundle {
                projectile_tower: ProjectileTower {
                    speed: 200.0,
                    damage: 20,
                    shoot_timer: Timer::from_seconds(5.0, TimerMode::Repeating), 
                },
                health: Health { health: 50 },
            }), "art/yeti.png".to_string())
        );
        map
    };
}

#[derive(Component)]
pub struct Preview(Tower); 

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .insert_resource(SelectableTowers {
                possible_towers: setup_tower_costs(),
                selected_tower: None,
            })
            .add_event::<MoneyUpdated>()
            .add_systems(Startup, setup)
            .add_systems(Update, (
                spawn_tower_at_mouse, update_money, select_tower, projectile_damage_enemies,
                shoot_projectiles, move_projectiles, despawn_out_of_bound_projectile, check_tower_health))
        ;
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    spawn_preview(commands, &asset_server, Tower::Money);
}

fn spawn_preview(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    tower: Tower
) {
    commands.spawn((
        get_sprite_bundle(&TOWER_IMPLEMENTATIONS[&tower].1, Vec3::default(), &asset_server, 0.0),
        Preview(tower)
    ));

}

fn setup_tower_costs() -> Vec<BuyableTower> {
    vec![
        BuyableTower {
            cost: 100,
            tower_type: Tower::Money
        },
        BuyableTower {
            cost: 200,
            tower_type: Tower::NormalProjectile
        },

    ]
}

fn select_tower(
    input: Res<Input<KeyCode>>,
    mut selectable_towers: ResMut<SelectableTowers>
) {
    let towers_length = selectable_towers.possible_towers.len();
    if input.just_pressed(KeyCode::Key1) && towers_length > 0 {
        selectable_towers.selected_tower = Some(selectable_towers.possible_towers[0].clone());
    } else if input.just_pressed(KeyCode::Key2) && towers_length > 1 {
        selectable_towers.selected_tower = Some(selectable_towers.possible_towers[1].clone());
    } else if input.just_pressed(KeyCode::Key3) && towers_length > 2 {
        selectable_towers.selected_tower = Some(selectable_towers.possible_towers[2].clone());
    } else if input.just_pressed(KeyCode::Key4) && towers_length > 3 {
        selectable_towers.selected_tower = Some(selectable_towers.possible_towers[3].clone());
    } else if input.just_pressed(KeyCode::Key5) && towers_length > 4 {
        selectable_towers.selected_tower = Some(selectable_towers.possible_towers[4].clone());
    }

}

fn spawn_tower_at_mouse(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    input: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut money: ResMut<Money>,
    mut money_updated: EventWriter<MoneyUpdated>,
    selected_tower: Res<SelectableTowers>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut preview_q: Query<(&mut Transform, &mut Sprite, Entity, &Preview)>
) {

    let (mut transform, mut sprite, preview_ent, preview) = preview_q.single_mut();
    sprite.color.set_a(0.0);
    if let Some(tower_cost) = &selected_tower.selected_tower {

        let (camera, camera_transform) = camera_q.single();

        let window = primary_query.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let grid_pos = convert_world_to_grid(Vec3 {
                x: world_position.x - GRID_X_SPACING / 2.0, y: world_position.y - GRID_Y_SPACING / 2.0, z: 0.0
            });
            if !board.towers.contains_key(&grid_pos) {
                return    
            }
            if board.towers[&grid_pos] != None {
                return;
            }
            else if tower_cost.cost > money.0 {
                return;
            }
            let spawn_pos = convert_grid_to_world(grid_pos);
            if input.just_pressed(MouseButton::Left) {
                money.0 -= tower_cost.cost;
                money_updated.send(MoneyUpdated{
                    new_value: money.0
                });


                let ent = spawn_tower(commands, spawn_pos, tower_cost, &asset_server);
                board.towers.insert(grid_pos, Some(ent));
            } else {
                if tower_cost.tower_type != preview.0 {
                    commands.entity(preview_ent).despawn();
                   spawn_preview(commands, &asset_server, tower_cost.tower_type); 
                } else {
                    transform.translation = spawn_pos;
                    sprite.color.set_a(0.7);
                }
            }
        }
    } else {
        return;
    }
}

fn spawn_tower(mut commands: Commands<'_, '_>, spawn_pos: Vec3, tower_cost: &BuyableTower, asset_server: &Res<AssetServer>) -> Entity {
    let texture_path = &TOWER_IMPLEMENTATIONS[&tower_cost.tower_type].1;
    let ent = match &TOWER_IMPLEMENTATIONS[&tower_cost.tower_type].0 {
        TowerBundle::Money(money_tower) => 
            commands.spawn((
                get_sprite_bundle(&texture_path, spawn_pos, asset_server, 1.0),
                money_tower.clone()
            )),
        TowerBundle::SimpleProjectile(projectile_tower) => 
            commands.spawn((
                get_sprite_bundle(&texture_path, spawn_pos, asset_server, 1.0),
                projectile_tower.clone()
            )),
    };
    ent.id()
}

fn get_sprite_bundle(texture_path: &str, spawn_pos: Vec3, asset_server: &Res<AssetServer>, alpha: f32) -> SpriteBundle {
    return SpriteBundle {
        texture: asset_server.load(texture_path),
        transform: Transform {
            translation: spawn_pos,
            ..Default::default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2::new(200.0, 150.0)),
            color: Color::rgba(1.0, 1.0, 1.0, alpha),
            ..default()
        },
        ..default()
    }
}

fn update_money(
    mut money: ResMut<Money>, 
    time: Res<Time>,
    mut towers: Query<&mut MoneyGain>,
    mut money_updated: EventWriter<MoneyUpdated>,
) {
    for mut tower in &mut towers {
        tower.gaintimer.tick(time.delta());

        if tower.gaintimer.finished() {
            money.0 += tower.money;
            money_updated.send(MoneyUpdated{
                new_value: money.0
            });
        }
    }

}

fn shoot_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut towers: Query<(&mut ProjectileTower, &Transform)>,
    enemies: Query<(&EnemyHealth, &Transform)>,
    asset_server: Res<AssetServer>
) {
    for (mut tower, transform) in &mut towers {
        tower.shoot_timer.tick(time.delta());
        let mut enemy_present = false;
        for (_, enemy_transform) in &enemies {
            if enemy_transform.translation.y == transform.translation.y {
               enemy_present = true;
               break; 
            }
        }
        if tower.shoot_timer.finished() && enemy_present {
            commands.spawn((
                get_sprite_bundle(
                    "kenney/PNG/DefaultSize/towerDefense_tile251.png", transform.translation, &asset_server, 1.0),
                Projectile {
                    speed: tower.speed,
                    damage: tower.damage
                }
            ));
        }
    }
}

fn move_projectiles(
    mut projectiles: Query<(&mut Transform, &Projectile)>,
    time: Res<Time>
) {
    for (mut transform, projectile) in projectiles.iter_mut() {
        transform.translation.x += projectile.speed * time.delta_seconds();
    }
}

fn despawn_out_of_bound_projectile(
    mut projectiles: Query<(&Transform, &Projectile, Entity)>,
    mut commands: Commands,
) {
    for (transform, _, projectile_ent) in projectiles.iter_mut() {
        if transform.translation.x > 550.0 {
            commands.entity(projectile_ent).despawn();
        }
    }
}

fn projectile_damage_enemies(
    mut commands: Commands,
    mut projectiles: Query<(&Transform, &Projectile, Entity)>,
    mut enemies: Query<(&mut EnemyHealth, &Transform)>
) {
    for (transform, projectile, projectile_ent) in projectiles.iter_mut() {
        for (mut enemy_health, enemy_transform ) in &mut enemies {
            if transform.translation.y == enemy_transform.translation.y && (transform.translation.x - enemy_transform.translation.x).abs() < 10.0 {
                enemy_health.health -= projectile.damage;
                commands.entity(projectile_ent).despawn();
            }
        }
    } 
}


fn check_tower_health(
    mut commands: Commands,
    query: Query<(&Health, Entity), Changed<Health>>,
) {
    for (health, entity) in query.iter() {
        if health.health <= 0 {
            commands.entity(entity).despawn();
        }
    }
}