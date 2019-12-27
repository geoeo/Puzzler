use crate::model::commands::MoveCommand;


pub fn simple_command(move_command: &MoveCommand) -> (i16,i16) {
    match move_command {
        MoveCommand::Left => (1,0),
        MoveCommand::Right => (-1,0),
        MoveCommand::Up => (0,-1),
        MoveCommand::Down=> (0,1),
        MoveCommand::None => (0,0)
    }
}


pub fn no_command(_: &MoveCommand) -> (i16,i16) {
    (0,0)
}
