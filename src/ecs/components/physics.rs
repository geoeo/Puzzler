use crate::model::commands::Move;

#[derive(Copy, Clone)]
pub struct Physics {
    pub repulsion_factor: i16,
    pub move_delegate: fn(move_value: Move, multiplier: i16) -> Move
}

impl std::fmt::Debug for Physics {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Physics")
            .field("repulsion_factor",&self.repulsion_factor)
            .finish()
    }
}