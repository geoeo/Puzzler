use crate::model::commands::{MoveCommand, Move};

//TODO: think about traits
pub fn simple_command(move_command: &MoveCommand, multiplier: i16) -> Move {
    match move_command {
        MoveCommand::Left => (-multiplier, 0),
        MoveCommand::Right => (multiplier, 0),
        MoveCommand::Up => (0, -multiplier),
        MoveCommand::Down => (0, multiplier),
        MoveCommand::None => (0, 0)
    }
}

pub fn no_command(_: &MoveCommand, _: i16) -> Move {
    (0,0)
}

pub fn simple_move((delta_x, delta_y): Move, multiplier: i16)  -> Move {
    (multiplier*delta_x,multiplier*delta_y)
}

pub fn no_move(_: Move, _: i16) -> Move {(0,0)}