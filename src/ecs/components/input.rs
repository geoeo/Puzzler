use crate::model::commands::MoveCommand;

#[derive(Copy, Clone)]
pub struct Input {
    pub keyboard_delegate: fn(&MoveCommand) -> (i16, i16)
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Input").finish()
    }
}
