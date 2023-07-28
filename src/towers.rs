use bevy::{prelude::*, window::PrimaryWindow};

use crate::{MainCamera, board::*};

#[derive(Resource)]
pub struct Money(pub u32);


#[derive(Event)]
pub struct MoneyUpdated {
    pub new_value: u32,
}

#[derive(Component)]
pub struct EnemyHealth {

}

#[derive(Component)]
pub struct MoneyTower {
    pub money: u32,
    pub gaintimer: Timer,
}

pub fn spawn_tower_at_mouse(
    commands: Commands, 
    asset_server: Res<AssetServer>, 
    input: Res<Input<KeyCode>>,
    mut board: ResMut<Board>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let (camera, camera_transform) = camera_q.single();

    let window = primary_query.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        let grid_pos = convert_world_to_grid(Vec3 {
            x: world_position.x - GRID_X_SPACING / 2.0, y: world_position.y - GRID_Y_SPACING / 2.0, z: 0.0
        });
        if board.towers[&grid_pos] != None {
            return;
        }

        let spawn_pos = convert_grid_to_world(grid_pos);

        let texture = asset_server.load("kenney/PNG/DefaultSize/towerDefense_tile249.png");

        let ent = spawn_tower(commands, texture, spawn_pos);
        board.towers.insert(grid_pos, Some(ent));
    }
}

fn spawn_tower(mut commands: Commands<'_, '_>, texture: Handle<Image>, spawn_pos: Vec3) -> Entity {
    let ent = commands.spawn((SpriteBundle {
        texture,
        transform: Transform {
            translation: spawn_pos,
            rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        ..default()
    },
    MoneyTower {
        money: 50, 
        gaintimer: Timer::from_seconds(3.0, TimerMode::Repeating),
    }
    )
    );
    ent.id()
}

pub fn update_money(
    mut money: ResMut<Money>, 
    time: Res<Time>,
    mut towers: Query<&mut MoneyTower>,
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