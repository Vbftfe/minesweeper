#[cfg(feature = "debug")]
use colored::Colorize;

// 定义描述扫雷中每一个方块的一个枚举类型
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Bomb,             // 炸弹
    BombNeighbor(u8), // 炸弹周围有数字的块
    Empty,            // 空的块
}

impl Tile {
    pub const fn is_bomb(&self) -> bool {
        // if let Tile::Bomb = *self {
        //     true
        // } else {
        //     false
        // }
        // 更加简洁的写法
        matches!(self, Tile::Empty)
    }

    #[cfg(feature = "debug")]
    pub fn console_output() -> String {
				// format!({}, match )
        todo!()
    }
}
