use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    log,
    prelude::{EventReader, MouseButton, Res, Windows},
};

use crate::resources::board::Board;

pub fn event_handle(
    board: Res<Board>,
    mut event_reader: EventReader<MouseButtonInput>,
    windows: Res<Windows>,
) {
    if let Some(event) = event_reader.iter().next() {
        if let ButtonState::Released = event.state {
            return;
        }
        let window = windows.get_primary().unwrap();
        let mouse_pos = window.cursor_position().unwrap();
        log::trace!("Mouse Buttion Pressed: {:?} as {}", event.button, mouse_pos);
        let coords = board.mouse_position(window, mouse_pos);
        if let None = coords {
            return;
        }
        let coords = coords.unwrap();
        match event.button {
            MouseButton::Left => {
                log::info!("Trying to uncover tile on {}", coords);
            }
            MouseButton::Right => {
                log::info!("Trying to mark tile on {}", coords);
            }
            _ => (),
        }
    }
}
