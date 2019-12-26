use crate::ecs::components::{identifier::Identifier,position::Position,display::Display,keyboard_input::KeyboardInput};
use array2d::Array2D;

const ENTITY_CAPACITY : usize = 50;

//Index from top left
#[derive(Debug,Clone)]
pub struct Level {
    pub width : usize,
    pub height: usize,
    occupancy: Vec<bool>,
    pub map: Array2D<Display>,
    identifiers: Vec<Identifier>,
    positions: Vec<Position>,
    keyboard_input: Vec<KeyboardInput>
}

impl Level {

    pub fn new(width: usize, height: usize) -> Level {
        Level{
            width: width,
            height: height,
            occupancy: vec![false; ENTITY_CAPACITY],
            map: Array2D::filled_with(Display::new('.'),height,width),
            identifiers: Vec::with_capacity(ENTITY_CAPACITY),
            positions: Vec::with_capacity(ENTITY_CAPACITY),
            keyboard_input: Vec::with_capacity(ENTITY_CAPACITY)
        }

    }

}

