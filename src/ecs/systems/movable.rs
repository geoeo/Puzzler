use crate::ecs::components::position::Position;
use crate::model::commands::MoveCommand;
use crate::ecs::systems::commandable::Commandable;
use crate::ecs::components::command::{CommandType, NoCommand, SimpleCommand};

pub trait Movable {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16) -> Position;
    fn apply_move_on_all(controllable: &mut Vec<Position>, command_types: &Vec<CommandType>, move_command: &MoveCommand) -> ();
}

impl Movable for Position {
    fn apply_delta(position: &Position, delta_x: i16, delta_y: i16) -> Position {
        let x_new = position.x_pos as i16 + delta_x;
        let y_new = position.y_pos as i16 + delta_y;
        assert!(x_new >= 0);
        assert!(y_new >= 0);

        Position {
            x_pos:x_new as u16,
            y_pos:y_new as u16
        }
    }

    fn apply_move_on_all(controllable: &mut Vec<Position>, command_types: &Vec<CommandType>, move_command: &MoveCommand) -> () {
        for i in 0..controllable.len() {
            let old_pos = controllable[i];
            let command_type = command_types[i];
            let(delta_x,delta_y) = match command_type {
                CommandType::NoCommand => NoCommand::apply_commmand(move_command),
                CommandType::SimpleCommand => SimpleCommand::apply_commmand(move_command)
            };
            let new_pos = Position::apply_delta(&old_pos,delta_x,delta_y);
            controllable[i] = new_pos;
        }
    }
}