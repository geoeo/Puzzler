use crossterm::event::{KeyCode};
use crate::model::commands::{InputCommand, MoveCommand};

pub fn process_input(input_command: &InputCommand) -> MoveCommand {
    if input_command.valid {
        match input_command.key_event.code {
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



