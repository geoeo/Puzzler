use crate::ecs::components::position::Position;

trait Movable {
    fn new_pos(position: &Position, delta_x: u16, delta_y: u16) -> Position;
}

impl Movable for Position {
    fn new_pos(position: &Position, delta_x: u16, delta_y: u16) -> Position {
        Position {
            x_pos: position.x_pos + delta_x,
            y_pos: position.y_pos + delta_y
        }
    }
}