use bevy::prelude::{Resource, Vec3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileSize {
    Fixed(f32),
    Adaptive { min: f32, max: f32 },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoardPosition {
    Centered { offset: Vec3 },
    Custom(Vec3),
}

#[derive(Debug, Clone, Serialize, Deserialize, Resource)]
pub struct BoardOptions {
    pub map_size: (u16, u16),
    pub bomb_count: u16,
    pub position: BoardPosition,
    pub tile_size: TileSize,
    pub tile_padding: f32,
    pub safe_start: bool,
}

impl Default for TileSize {
    fn default() -> Self {
        TileSize::Adaptive { min: 10., max: 50. }
    }
}

impl Default for BoardPosition {
    fn default() -> Self {
        Self::Centered {
            offset: Vec3::default(),
        }
    }
}

impl Default for BoardOptions {
    fn default() -> Self {
        BoardOptions {
            map_size: (15, 15), // (x, y)
            bomb_count: 30,
            position: BoardPosition::default(),
            tile_size: TileSize::default(),
            tile_padding: 0.,
            safe_start: false,
        }
    }
}
