use crate::ecs::components::command::{SimpleCommand, NoCommand};
use crate::model::commands::MoveCommand;

pub trait Commandable {
    fn apply_commmand(move_command: &MoveCommand) -> (i16,i16);
}

impl Commandable for SimpleCommand {
    fn apply_commmand(move_command: &MoveCommand) -> (i16,i16) {
        match move_command {
            MoveCommand::Left => (1,0),
            MoveCommand::Right => (-1,0),
            MoveCommand::Up => (0,-1),
            MoveCommand::Down=> (0,1),
            MoveCommand::None => (0,0)
        }
    }
}

impl Commandable for NoCommand {
    fn apply_commmand(move_command: &MoveCommand) -> (i16,i16) {
        (0,0)
    }
}