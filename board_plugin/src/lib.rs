pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::{tile_map::TileMap, BoardOptions};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(BoardPlugin::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    pub fn create_board(mut commands: Commands) {
        let mut tile_map = TileMap::new(20, 20);
        tile_map.set_bombs(40);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}
