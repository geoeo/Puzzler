use crate::model::commands::MoveCommand;

#[derive(Copy, Clone)]
pub struct Position {
    pub x_pos: u16,
    pub y_pos: u16,
    pub input_delegate: fn(&MoveCommand) -> (i16,i16)
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Position")
            .field("x_pos", &self.x_pos)
            .field("y_pos", &self.y_pos)
            .finish()
    }
}


