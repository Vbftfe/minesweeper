pub mod components;
pub mod resources;

use bevy::log;
use bevy::prelude::*;
use resources::BoardPosition;
use resources::TileSize;
use resources::{tile_map::TileMap, BoardOptions};

use crate::components::{Bomb, BombNeighbor, Coordinates, Uncover};

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(BoardPlugin::create_board);
        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    fn build(&self, app: &mut App) {
        // ..
        #[cfg(feature = "debug")]
        {
            // registering custom component to be able to edit it in inspector
            app.register_inspectable::<Coordinates>();
            app.register_inspectable::<BombNeighbor>();
            app.register_inspectable::<Bomb>();
            app.register_inspectable::<Uncover>();
        }
    }

    pub fn create_board(
        mut commands: Commands,
        board_options: Res<BoardOptions>,
        windows: Res<Windows>,
        assets_server: Res<AssetServer>,
    ) {
        let map_size = board_options.map_size;
        let mut tile_map = TileMap::new(map_size.0, map_size.1);
        let window = windows.get_primary().unwrap();
        let font = assets_server.load("fonts/pixeled.ttf");
        let bomb_png = assets_server.load("sprites/bomb.png");
        // 设定炸弹数目
        tile_map.set_bombs(board_options.bomb_count);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let tile_size = actual_tile_size(
            (window.width(), window.height()),
            &board_options.tile_size,
            map_size,
        );

        #[cfg(feature = "debug")]
        log::info!("tile size is {}", tile_size);

        // 计算board的中心位置
        let (board_width, board_height) =
            (tile_size * map_size.0 as f32, tile_size * map_size.1 as f32);
        let board_position = board_position((board_width, board_height), board_options.position);

        // 创建board
        commands
            .spawn_empty()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .insert(ComputedVisibility::default())
            .insert(Visibility { is_visible: true }) // 两个Visibility用来将父级的空元素显示，否则所有子元素都不能显示
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(Vec2::new(board_width, board_height)),
                            ..default()
                        },
                        transform: Transform::from_xyz(board_width / 2., board_height / 2., 0.),
                        ..default()
                    })
                    .insert(Name::new("Background"));
            })
            .with_children(|parent| {
                // 创建tile
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        parent
                            .spawn(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(Vec2::splat(
                                        tile_size - board_options.tile_padding,
                                    )),
                                    ..default()
                                },
                                transform: Transform::from_xyz(
                                    tile_size * x as f32 + tile_size / 2.,
                                    tile_size * y as f32 + tile_size / 2.,
                                    1.,
                                ),
                                ..default()
                            })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }
}

// 根据当前窗口的大小以及设定的tile_size计算具体每个tile的实际大小
fn actual_tile_size(
    (window_width, window_height): (f32, f32),
    tile_size: &TileSize,
    map_size: (u16, u16),
) -> f32 {
    match *tile_size {
        TileSize::Fixed(size) => size,
        TileSize::Adaptive { min, max } => {
            let tile_width = window_width / map_size.0 as f32;
            let tile_height = window_height / map_size.1 as f32;
            let size = f32::min(tile_width, tile_height);
            size.clamp(min, max)
        }
    }
}

fn board_position((board_width, board_height): (f32, f32), board_position: BoardPosition) -> Vec3 {
    match board_position {
        BoardPosition::Custom(vec3) => vec3,
        BoardPosition::Centered { offset } => Vec3::new(
            -board_width / 2. + offset.x,
            -board_height / 2. + offset.y,
            0.,
        ),
    }
}
