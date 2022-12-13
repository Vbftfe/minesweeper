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
    pub fn console_output(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(num) => match num {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => num.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }
}

#[cfg(feature = "debug")]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tile = Tile::BombNeighbor(4);

        println!("{}", tile.console_output());
    }
}
