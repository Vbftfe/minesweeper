use super::tile::Tile;
use crate::components::coordinates::Coordinates;
use std::ops::{Deref, DerefMut};

const SQUARE_COORDINATES: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub struct TileMap {
    bomb_count: u16,
    height: u16,
    width: u16,
    map: Vec<Vec<Tile>>,
}

impl TileMap {
    pub fn new(height: u16, width: u16) -> Self {
        let map = (0..height)
            .map(|_| (0..width).map(|_| Tile::Empty).collect())
            .collect();

        Self {
            bomb_count: 0,
            height,
            width,
            map,
        }
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn bomb_count(&self) -> u16 {
        self.bomb_count
    }

    // 获取某个坐标周围的8个位置的坐标
    pub fn safe_square_at(&self, position: Coordinates) -> impl Iterator<Item = Coordinates> {
        SQUARE_COORDINATES
            .iter()
            .copied()
            .map(move |coord| position + coord)
    }

    pub fn is_bombs_at(&self, position: Coordinates) -> bool {
        if position.x > self.width || position.y > self.height {
            return false;
        }
        self.map[position.y as usize][position.x as usize].is_bomb()
    }

    pub fn bomb_count_at(&self, position: Coordinates) -> u8 {
        if self.is_bombs_at(position) {
            return 0;
        }
        self.safe_square_at(position)
            .filter(|coord| self.is_bombs_at(*coord))
            .count() as u8
    }

    #[cfg(feature = "debug")]
    pub fn console_output(&self) -> String {
        let mut buffer = format!(
            "Map ({}, {}) with {} bombs:\n",
            self.width, self.height, self.bomb_count
        );
        let line = (0..(self.width + 1)).map(|_| '-').collect::<String>();
        buffer = format!("{}{}\n", buffer, line);

        for line in self.iter() {
            buffer = format!("{}|", buffer);
            for tile in line.iter() {
                buffer = format!("{}{}", buffer, tile.console_output());
            }
            buffer = format!("{}|\n", buffer);
        }
        format!("{}{}", buffer, line)
    }
}

impl Deref for TileMap {
    type Target = Vec<Vec<Tile>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for TileMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
