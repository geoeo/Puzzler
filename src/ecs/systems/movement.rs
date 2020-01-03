use crate::ecs::components::position::Position;
use crate::ecs::components::input::Input;
use crate::model::commands::{MoveCommand, Move};
use crate::ecs::components::physics::Physics;


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

pub fn apply_input_move(positions: &mut Vec<Option<Position>>, inputs: &Vec<Option<Input>>, current_moves: &mut Vec<Option<Move>>, level_width: u16, level_height: u16, move_command: &MoveCommand, multiplier: i16) -> () {
    for i in 0..positions.len() {
        match (positions[i], inputs[i]) {
            (Some(old_pos), Some(input)) => {
                let (delta_x, delta_y) = (input.keyboard_delegate)(&move_command, multiplier);
                let new_pos = apply_delta(&old_pos, delta_x, delta_y, level_width, level_height);
                positions[i] = Some(new_pos);
                current_moves[i] = Some((delta_x,delta_y));
            },
            _ => continue
        }
    }
}

pub fn apply_physics(physics: &Vec<Option<Physics>>, positions: &mut Vec<Option<Position>>, current_moves: &mut Vec<Option<Move>>, level_width: u16, level_height: u16) -> () {
    for i in 0..physics.len() {
        match (physics[i], positions[i],current_moves[i]) {
            (Some(physics_handler), Some(position), Some(move_value) )=> {
                let (delta_x,delta_y) = (physics_handler.move_delegate)(&move_value);
                let new_pos = apply_delta(&position, delta_x, delta_y, level_width, level_height);
                positions[i] = Some(new_pos);
                current_moves[i] = Some((delta_x,delta_y));
            },
            _ => continue
        }
    }
}
