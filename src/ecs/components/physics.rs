use crate::model::commands::MoveCommand;
use crate::ecs::components::position::Position;

#[derive(Copy, Clone)]
pub struct Physics {
    pub move_delegate: fn(move_command: &MoveCommand, position: &Position) -> (MoveCommand)
}

impl std::fmt::Debug for Physics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Physics").finish()
    }
}