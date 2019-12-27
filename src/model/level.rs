use crate::ecs::components::{
    identifier::Identifier,
    position::Position,
    occupancy::Occupancy};
use array2d::Array2D;

const ENTITY_CAPACITY : usize = 50;

//TODO: refactor this data structure
//Index from top left
#[derive(Debug,Clone)]
pub struct Level {
    pub width : usize,
    pub height: usize,
    pub map: Array2D<Option<u64>>,
    pub occupancies: Vec<Occupancy>,
    pub identifiers: Vec<Option<Identifier>>,
    pub positions: Vec<Option<Position>>
}

impl Level {

    pub fn new(width: usize, height: usize) -> Level {
        Level{
            width: width,
            height: height,
            occupancies: vec![Occupancy::new(); ENTITY_CAPACITY],
            map: Array2D::filled_with(None,height,width),
            identifiers: vec![None;ENTITY_CAPACITY],
            positions: vec![None;ENTITY_CAPACITY]
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

    pub fn update_map(&self)->(){
        //TODO:
    }

}

