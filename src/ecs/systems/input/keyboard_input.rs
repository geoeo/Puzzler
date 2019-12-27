use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers}
};
use crate::ecs::components::keyboard_input::KeyboardInput;
use crate::ecs::systems::input::Input;
use crate::model::commands::{InputCommand, MoveCommand};
use crate::model::game_state::GameState;

impl Input for KeyboardInput {
    // encapsulate this in an object with different traits depending on input type
    fn process_input(game_command: &InputCommand) -> MoveCommand {
        if game_command.valid {
            match game_command.key_event.code {
                KeyCode::Left => MoveCommand::Left,
                KeyCode::Right => MoveCommand::Right,
                KeyCode::Up => MoveCommand::Up,
                KeyCode::Down => MoveCommand::Down,
                KeyCode::Char('a') => MoveCommand::Left,
                KeyCode::Char('d') => MoveCommand::Right,
                KeyCode::Char('w') => MoveCommand::Up,
                KeyCode::Char('s') => MoveCommand::Down,
                _ => MoveCommand::None
            }
        } else {
            MoveCommand::None
        }
    }

    fn parse_input_event(event: &Event) -> (GameState, InputCommand) {

        match event {
            Event::Key(keyboard) => {
                (match keyboard.code {
                    KeyCode::Esc => GameState::QuitGame,
                    _ => GameState::Running
                }, InputCommand {valid: false, key_event: keyboard.clone()})
            },
            _ => (GameState::Running, InputCommand {valid: false, key_event: KeyEvent{code: KeyCode::Esc, modifiers: KeyModifiers::empty()}})
        }

    }
}


