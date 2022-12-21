use super::tile::Tile;
use crate::components::coordinates::Coordinates;
use rand::{thread_rng, Rng};
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

#[derive(Debug)]
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

    #[allow(unused)]
    pub fn height(&self) -> u16 {
        self.height
    }

    #[allow(unused)]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[allow(unused)]
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
        if position.x >= self.width || position.y >= self.height {
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
        let line = (0..=(self.width + 1)).map(|_| '-').collect::<String>();
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

// 实现炸弹的布置以及数字提示的生成
impl TileMap {
    pub fn set_bombs(&mut self, bomb_num: u16) {
        let mut rng = thread_rng();
        let mut _remain_bombs_num = bomb_num;
        self.bomb_count = bomb_num;

        // 放置炸弹
        while _remain_bombs_num > 0 {
            let (x, y): (u16, u16) = (rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            let tile = &mut self.map[y as usize][x as usize];
            if let Tile::Empty = *tile {
                *tile = Tile::Bomb;
                _remain_bombs_num -= 1;
            }
        }

        // 设置炸弹周围的数字
        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                let position = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                if self.is_bombs_at(position) {
                    continue;
                }
                let neighbor_value = self.bomb_count_at(position);
                if neighbor_value == 0 {
                    continue;
                } else {
                    self.map[y][x] = Tile::BombNeighbor(neighbor_value);
                }
            }
        }
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
