use crate::model::commands::MoveCommand;


pub fn simple_command(move_command: &MoveCommand, multiplier: i16) -> (i16,i16) {
    match move_command {
        MoveCommand::Left => (-multiplier, 0),
        MoveCommand::Right => (multiplier, 0),
        MoveCommand::Up => (0, -multiplier),
        MoveCommand::Down => (0, multiplier),
        MoveCommand::None => (0, 0)
    }
}


pub fn no_command(_: &MoveCommand) -> (i16,i16) {
    (0,0)
}
