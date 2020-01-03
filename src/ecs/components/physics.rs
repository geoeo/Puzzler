use crate::model::commands::Move;

#[derive(Copy, Clone)]
pub struct Physics {
    pub move_delegate: fn(move_value: &Move) -> Move
}

impl std::fmt::Debug for Physics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Physics").finish()
    }
}