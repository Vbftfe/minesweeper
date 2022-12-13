use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use bevy::prelude::Component;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Coordinates {
    pub x: u16,
    pub y: u16,
}

impl Add for Coordinates {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinates {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(i8, i8)> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: (i8, i8)) -> Self::Output {
        Coordinates {
            // 注意处理越界的情况
            // 通过下面的处理方式可以将负数都转为i16::MAX加上这个负数，考虑到游戏的格子数不会太大，因此在后续的处理中可以通过判断是否大于限定的高度和宽度来去除
            x: (self.x as i16 + rhs.0 as i16) as u16,
            y: (self.y as i16 + rhs.1 as i16) as u16,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let num = -2i16;
        println!("i16 number {} as u16 is {}", num, num as u16);
    }
}
