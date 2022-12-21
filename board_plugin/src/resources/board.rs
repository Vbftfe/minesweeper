use super::tile_map::TileMap;
use crate::{bounds::Bounds2, components::Coordinates};
use bevy::prelude::{Resource, Vec2, Window};

#[derive(Debug, Resource)]
pub struct Board {
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub tile_map: TileMap,
}

impl Board {
    // 将鼠标的坐标转化成以board左下角为原点的坐标
    // 若鼠标点击的位置不在board中则返回None
    pub fn mouse_position(&self, window: &Window, mouse_pos: Vec2) -> Option<Coordinates> {
        let win_width = window.width();
        let win_height = window.height();
        // 计算mouse相对于窗口左下角的位置
        let window_mouse_pos = mouse_pos - Vec2::new(win_width / 2., win_height / 2.);
        if self.bounds.in_bounds(window_mouse_pos) {
            // 计算mouse相对于board左下角的位置
            let board_mouse_pos = window_mouse_pos - self.bounds.position;
            Some(Coordinates {
                x: (board_mouse_pos.x / self.tile_size) as u16,
                y: (board_mouse_pos.y / self.tile_size) as u16,
            })
        } else {
            None
        }
    }
}
