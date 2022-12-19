use bevy::prelude::Component;

pub mod coordinates;

pub use coordinates::Coordinates;

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Bomb;
#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BombNeighbor {
    pub count: u8,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Component, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Uncover;
