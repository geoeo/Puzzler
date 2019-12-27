use crate::ecs::components::position::Position;
use crate::model::commands::MoveCommand;

pub trait Movable {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16) -> Position;
    fn apply_move_on_all(controllable: &mut Vec<Option<Position>>, move_command: &MoveCommand) -> ();
}

//TODO: check level boundaries!!
impl Movable for Position {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16) -> Position {
        let x_new = position.x_pos as i16 + delta_x;
        let y_new = position.y_pos as i16 + delta_y;
        assert!(x_new >= 0);
        assert!(y_new >= 0);

        Position {
            x_pos:x_new as u16,
            y_pos:y_new as u16,
            input_delegate: position.input_delegate
        }
    }

    fn apply_move_on_all(controllable: &mut Vec<Option<Position>>, move_command: &MoveCommand) -> () {
        for i in 0..controllable.len() {
            match controllable[i] {
                Some(old_pos) => {
                    let (delta_x, delta_y) = (old_pos.input_delegate)(&move_command);
                    let new_pos = Position::apply_delta(&old_pos,delta_x,delta_y);
                    controllable[i] = Some(new_pos);
                },
                None => continue
            }
        }
    }
}