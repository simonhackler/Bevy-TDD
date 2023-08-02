use std::collections::HashMap;

use bevy::prelude::*;


pub const GRID_Y_SPACING: f32 = 80.0;
pub const GRID_X_SPACING: f32 = 80.0;

#[derive(Resource)]
pub struct Board {
    pub towers: HashMap<(i32,i32), Option<Entity>>,
}


pub fn generate_board() -> Board {
    let mut board = Board {
        towers: HashMap::new(),
    };
    for i in -7..7{
        for j in -4..3 {
            board.towers.insert((i as i32, j as i32), None);
        }
    }
    return board
}

pub fn convert_grid_to_world(grid: (i32,i32)) -> Vec3 {
    return Vec3::new(grid.0 as f32 * GRID_X_SPACING + GRID_X_SPACING / 2.0, grid.1 as f32 * GRID_Y_SPACING + GRID_Y_SPACING / 2.0,0.0 );
}

pub fn convert_world_to_grid(world: Vec3) -> (i32,i32) {
    let x = (world.x / GRID_X_SPACING as f32).round() as i32;
    let y = (world.y / GRID_Y_SPACING as f32).round() as i32;
    return (x, y);
}