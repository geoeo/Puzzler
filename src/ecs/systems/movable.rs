use crate::ecs::components::position::Position;
use crate::model::commands::MoveCommand;
use crate::ecs::components::input::Input;

pub trait Movable {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16, level_width: u16, level_height: u16) -> Position;
    fn apply_move_on_all(controllable: &mut Vec<Option<Position>>, inputs: &Vec<Option<Input>>, level_width: u16, level_height: u16, move_command: &MoveCommand) -> ();
}


impl Movable for Position {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16, level_width: u16, level_height: u16) -> Position {
        let x_new = match position.x_pos as i16 + delta_x {
            x if x < 0 => 0,
            x if x >= level_width as i16 => level_width - 1,
            x => x as u16
        };

        let y_new = match position.y_pos as i16 + delta_y {
            y if y < 0 => 0,
            y if y >= level_width as i16 => level_height - 1,
            y => y as u16
        };

        Position {
            x_pos:x_new,
            y_pos:y_new
        }
    }

    fn apply_move_on_all(positions: &mut Vec<Option<Position>>, inputs: &Vec<Option<Input>>, level_width: u16, level_height: u16, move_command: &MoveCommand) -> () {
        for i in 0..positions.len() {
            match (positions[i], inputs[i]) {
                (Some(old_pos), Some(input)) => {
                    let (delta_x, delta_y) = (input.keyboard_delegate)(&move_command);
                    let new_pos = Position::apply_delta(&old_pos, delta_x, delta_y, level_width, level_height);
                    positions[i] = Some(new_pos);
                },
                _ => continue
            }
        }
    }
}