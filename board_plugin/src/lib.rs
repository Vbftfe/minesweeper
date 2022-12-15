pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::BoardPosition;
use resources::TileSize;
use resources::{tile_map::TileMap, BoardOptions};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(BoardPlugin::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOptions>,
        windows: Res<Windows>,
    ) {
        let map_size = board_options.map_size;
        let mut tile_map = TileMap::new(map_size.0, map_size.1);
        let window = windows.get_primary().unwrap();
        tile_map.set_bombs(board_options.bomb_count);
        let tile_size = actual_tile_size(
            (window.width(), window.height()),
            &board_options.tile_size,
            map_size,
        );

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}

// 根据当前窗口的大小以及设定的tile_size计算具体每个tile的实际大小
fn actual_tile_size(
    (window_width, window_height): (f32, f32),
    tile_size: &TileSize,
    map_size: (u16, u16),
) -> f32 {
    match *tile_size {
        TileSize::Fixed(size) => size,
        TileSize::Adaptive { min, max } => {
            let tile_width = window_width / map_size.0 as f32;
            let tile_height = window_height / map_size.1 as f32;
            let size = f32::min(tile_width, tile_height);
            size.clamp(min, max)
        }
    }
}

fn board_position(
    (board_width, board_height): (f32, f32),
    (window_width, window_height): (f32, f32),
    board_position: BoardPosition,
) -> BoardPosition {
    match board_position {
        BoardPosition::Custom(vec3) => BoardPosition::Custom(vec3),
        BoardPosition::Centered { offset: _ } => todo!(), //Vec3::new(, , 0.),
    }
}
