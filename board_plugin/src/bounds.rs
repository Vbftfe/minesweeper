use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bounds2 {
    pub position: Vec2, // 左下角为参考点
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.x >= self.position.x
            && position.x <= self.position.x + self.size.x
            && position.y >= self.position.y
            && position.y <= self.position.y + self.size.y
    }
}
