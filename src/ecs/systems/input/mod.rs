use crossterm::event::Event;

use crate::model::commands::{InputCommand, MoveCommand};
use crate::model::game_state::GameState;

pub mod keyboard_input;

pub trait Input {
    fn process_input(&self, game_command: &InputCommand) -> MoveCommand;
    fn parse_input_event(&self, event: &Event) -> (GameState, InputCommand);
}