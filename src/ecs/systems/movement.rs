use crate::ecs::components::position::Position;
use crate::ecs::components::tile::Tile;
use crate::ecs::components::input::Input;
use crate::model::commands::MoveCommand;
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

pub fn apply_move_on_all(positions: &mut Vec<Option<Position>>, inputs: &Vec<Option<Input>>,map: &mut Array2D<Tile>, level_width: u16, level_height: u16, move_command: &MoveCommand, multiplier: i16) -> () {
    for i in 0..positions.len() {
        match (positions[i], inputs[i]) {
            (Some(old_pos), Some(input)) => {
                let (delta_x, delta_y) = (input.keyboard_delegate)(&move_command, multiplier);
                let new_pos = apply_delta(&old_pos, delta_x, delta_y, level_width, level_height);
                match map.get_mut(new_pos.y_pos as usize,new_pos.x_pos as usize) {
                    Some(tile) => tile.new_ids.push(i as u64),
                    None => println!("Set new pos failed on tile {:?}", new_pos)
                }
                positions[i] = Some(new_pos);
            },
            _ => continue
        }
    }
}
