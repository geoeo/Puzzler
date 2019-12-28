use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers}
};
use crate::model::game_state::GameState;
use crate::model::commands::InputCommand;

pub mod keyboard_input;

pub fn parse_input_event(event: &Event) -> (GameState, InputCommand) {

    match event {
        Event::Key(keyboard) => {
            (match keyboard.code {
                KeyCode::Esc => GameState::QuitGame,
                _ => GameState::Running
            }, InputCommand {valid: true, key_event: keyboard.clone()})
        },
        _ => (GameState::Running, InputCommand {valid: false, key_event: KeyEvent{code: KeyCode::Esc, modifiers: KeyModifiers::empty()}})
    }

}
