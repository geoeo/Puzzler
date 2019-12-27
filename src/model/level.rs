use crate::ecs::components::{
    identifier::Identifier,
    position::Position,
    display::Display,
    occupancy::Occupancy};
use array2d::Array2D;

const ENTITY_CAPACITY : usize = 50;

//TODO: refactor this data structure
//Index from top left
#[derive(Debug,Clone)]
pub struct Level {
    pub width : usize,
    pub height: usize,
    pub map: Array2D<Display>, // Needs display and id
    pub occupancies: Vec<Occupancy>, //TODO: might remove
    pub identifiers: Vec<Identifier>,
    pub positions: Vec<Position>
}

impl Level {

    pub fn new(width: usize, height: usize) -> Level {
        Level{
            width: width,
            height: height,
            occupancies: vec![Occupancy::new(); ENTITY_CAPACITY],
            map: Array2D::filled_with(Display::new('.'),height,width),
            identifiers: Vec::with_capacity(ENTITY_CAPACITY),
            positions: Vec::with_capacity(ENTITY_CAPACITY)
        }

    }

    pub fn available_id(&self) -> Option<usize> {
        let mut id_option = None;
        for i in 0..self.occupancies.capacity() {
            if self.occupancies[i].is_free() {
                id_option = Some(i);
                break;
            };
        };

        id_option
    }

}

