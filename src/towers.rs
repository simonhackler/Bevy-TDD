use bevy::{prelude::*, window::PrimaryWindow, core_pipeline::core_2d::graph::input};

use crate::{MainCamera, board::*};


#[derive(Resource)]
pub struct Money(pub u32);

#[derive(Resource)]
pub struct SelectableTowers {
    pub possible_towers: Vec<TowerCost>,
    pub selected_tower: Option<TowerCost>,
}

#[derive(Event)]
pub struct MoneyUpdated {
    pub new_value: u32,
}

#[derive(Component, Clone, Debug)]
pub struct MoneyTower {
    pub money: u32,
    pub gaintimer: Timer,
}

#[derive(Clone, Debug)]
pub enum TowerType {
    Money(MoneyTower),
}

#[derive(Clone)]
pub struct TowerCost {
    pub cost: u32,
    pub texture_path: String,
    pub tower_type: TowerType,
}

pub fn setup_tower_costs() -> Vec<TowerCost> {
    vec![
        TowerCost {
            cost: 50,
            texture_path: "kenney/PNG/DefaultSize/towerDefense_tile249.png".to_string(),
            tower_type: TowerType::Money(MoneyTower {
                money: 50, 
                gaintimer: Timer::from_seconds(5.0, TimerMode::Repeating),
            }),
        },
    ]
}

pub fn select_tower(
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

pub fn spawn_tower_at_mouse(
    commands: Commands, 
    asset_server: Res<AssetServer>, 
    input: Res<Input<MouseButton>>,
    mut board: ResMut<Board>,
    mut money: ResMut<Money>,
    mut money_updated: EventWriter<MoneyUpdated>,
    selected_tower: Res<SelectableTowers>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {

    if !input.just_pressed(MouseButton::Left) {
        return;
    }

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
            if board.towers[&grid_pos] != None {
                return;
            }
            else if tower_cost.cost > money.0 {
                return;
            }
            money.0 -= tower_cost.cost;
            money_updated.send(MoneyUpdated{
                new_value: money.0
            });

            let spawn_pos = convert_grid_to_world(grid_pos);

            let ent = spawn_tower(commands, spawn_pos, tower_cost, asset_server);
            board.towers.insert(grid_pos, Some(ent));
        }
    } else {
        return;
    }


}

fn spawn_tower(mut commands: Commands<'_, '_>, spawn_pos: Vec3, tower_cost: &TowerCost, asset_server: Res<AssetServer>) -> Entity {
    let tower = match &tower_cost.tower_type {
        TowerType::Money(money_tower) => money_tower.clone(),
    };
    

    let ent = commands.spawn((
        get_sprite_bundle(&tower_cost.texture_path, spawn_pos, asset_server), 
        tower
    ));
    ent.id()
}


fn get_sprite_bundle(texture_path: &str, spawn_pos: Vec3, asset_server: Res<AssetServer>) -> SpriteBundle {
    return SpriteBundle {
        texture: asset_server.load(texture_path),
        transform: Transform {
            translation: spawn_pos,
            rotation: Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2),
            scale: Vec3::new(1.0, 1.0, 1.0),
        },
        ..default()
    }
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