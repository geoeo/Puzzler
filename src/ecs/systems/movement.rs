use crate::ecs::components::{position::Position,input::Input,tile::Tile, physics::Physics};
use crate::model::commands::{MoveCommand, Move};
use array2d::Array2D;

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

pub fn apply_physics(physics: &Vec<Option<Physics>>, positions: &mut Vec<Option<Position>>, current_moves: &mut Vec<Option<Move>>,map: &Array2D<Tile>,level_width: u16, level_height: u16) -> () {
    for i in 0..positions.len() {
        match (positions[i],current_moves[i]) {
            (Some(position), Some(move_value))=> {
                match map.get(position.y_pos as usize,position.x_pos as usize){
                    Some(tile) => {
                        for j in 0..tile.current_ids.len() {
                            let current_id = tile.current_ids[j];

                            match physics[current_id] {
                                Some(physics_handler) => {
                                    let (delta_x,delta_y) = (physics_handler.move_delegate)(move_value, physics_handler.repulsion_factor);
                                    let new_pos = apply_delta(&position, delta_x, delta_y, level_width, level_height);
                                    positions[i] = Some(new_pos);
                                    //TODO: Maybe not add it to current_moves
                                    current_moves[i] = Some((delta_x,delta_y));
                                },
                                None => continue
                            }

                        }
                    },
                    None => panic!("Apply Physics: Getting tile at position: {:?} failed!", position)
                }
            },
            _ => continue
        }
    }
}
