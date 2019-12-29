use crate::ecs::components::{
    display::Display,
    position::Position,
    occupancy::Occupancy,
    input::Input};
use array2d::Array2D;
use crate::ecs::components::debug_information::DebugInformation;

const ENTITY_CAPACITY : usize = 50;

//TODO: refactor this data structure
//Index from top left
#[derive(Debug,Clone)]
pub struct Level {
    pub width : u16,
    pub height: u16,
    pub map: Array2D<Option<u64>>,
    pub occupancies: Vec<Occupancy>,
    pub display: Vec<Option<Display>>,
    pub positions: Vec<Option<Position>>,
    pub debug: Vec<Option<DebugInformation>>,
    pub inputs: Vec<Option<Input>>
}

impl Level {

    pub fn new(width: u16, height: u16) -> Level {
        Level{
            width: width,
            height: height,
            occupancies: vec![Occupancy::new(); ENTITY_CAPACITY],
            map: Array2D::filled_with(None,height as usize,width as usize),
            display: vec![None; ENTITY_CAPACITY],
            positions: vec![None;ENTITY_CAPACITY],
            debug: vec![None;ENTITY_CAPACITY],
            inputs: vec![None;ENTITY_CAPACITY]
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

    pub fn clear_map(&mut self)->() {
        for x in 0..self.width {
            for y in 0..self.height {
                self.map.set(y as usize, x as usize, None).unwrap_or_else(|e| {
                    panic!("Clear map failed with error: {:?}", e);
                })
            }
        }
    }

    pub fn update_map(&mut self) -> () {
        for i in 0..self.positions.len() {
            match self.positions[i] {
                Some(pos) => match self.map.set(pos.y_pos as usize, pos.x_pos as usize, Some(i as u64)) {
                    Ok(_) => continue,
                    Err(e) => println!("Update map failed on {:?}", e)
                },
                None => continue
            }
        };
    }


    pub fn add_position(&mut self, display: Display, position: Position, debug: DebugInformation) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug.clone());
                self.occupancies[id] = Occupancy {position: true, display: true, input:false, debug: true};
                true
            },
            None => false
        }
    }

    pub fn add_position_input(&mut self, display: Display, position: Position,input: Input, debug: DebugInformation) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug);
                self.inputs[id] = Some(input);
                self.occupancies[id] = Occupancy {position: true, display: true, input:true, debug: true};
                true
            },
            None => false
        }
    }

    pub fn delete(&mut self, id: usize) -> bool {
        match id {
            id if id > self.occupancies.capacity() => false,
            id => {
                self.positions[id] = None;
                self.display[id] = None;
                self.inputs[id] = None;
                self.debug[id] = None;
                self.occupancies[id] = Occupancy::new();
                true
            }

        }
    }



}

