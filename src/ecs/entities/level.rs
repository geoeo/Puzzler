use crate::ecs::components::{
    display::Display,
    position::Position,
    occupancy::Occupancy,
    input::Input,
    tile::Tile,
    physics::Physics};
use array2d::Array2D;
use crate::ecs::components::debug_information::Debug;
use crate::model::commands::Move;

const ENTITY_CAPACITY : usize = 50;

//TODO: refactor this data structure
//Index from top left
#[derive(Debug,Clone)]
pub struct Level {
    pub width : u16,
    pub height: u16,
    pub map: Array2D<Tile>,
    pub occupancies: Vec<Occupancy>,
    pub display: Vec<Option<Display>>,
    pub positions: Vec<Option<Position>>,
    pub debug: Vec<Option<Debug>>,
    pub inputs: Vec<Option<Input>>,
    pub physics: Vec<Option<Physics>>,
    pub current_moves: Vec<Option<Move>> // This are the move values for the current frame; get cleared
}

impl Level {

    pub fn new(width: u16, height: u16) -> Level {
        Level{
            width: width,
            height: height,
            occupancies: vec![Occupancy::new(); ENTITY_CAPACITY],
            map: Array2D::filled_with(Tile::new(ENTITY_CAPACITY),height as usize,width as usize),
            display: vec![None; ENTITY_CAPACITY],
            positions: vec![None;ENTITY_CAPACITY],
            debug: vec![None;ENTITY_CAPACITY],
            inputs: vec![None;ENTITY_CAPACITY],
            physics: vec![None;ENTITY_CAPACITY],
            current_moves: vec![None;ENTITY_CAPACITY]
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

    pub fn clear(&mut self) ->() {
        for i in 0..self.current_moves.len() {
            self.current_moves[i] = None;
        }

        for x in 0..self.width {
            for y in 0..self.height {
                match self.map.get_mut(y as usize, x as usize) {
                    Some(tile) => {
                        tile.clear_current();
                    },
                    None => println!("Clear map failed on tile (x: {:?},y: {:?})", x, y)
                }

            }
        }
    }

    pub fn update_position_components(&mut self) -> () {
        for i in 0..self.positions.len() {
            match self.positions[i] {
                Some(position) => match self.map.get_mut(position.y_pos as usize, position.x_pos as usize) {
                    Some(tile) => {
                        tile.current_ids.push(i);
                    },
                    None => println!("Update position components failed on tile {:?}", position)
                },
                None => continue
            }
        };
    }

    pub fn update_input_components(&mut self) -> () {
        for i in 0..self.inputs.len() {
            match (self.positions[i], self.inputs[i]) {
                (Some(position), Some(_)) => match self.map.get_mut(position.y_pos as usize, position.x_pos as usize) {
                    Some(tile) => {
                        tile.current_ids.push(i);
                    },
                    None => println!("Update input components failed on tile {:?}", position)
                },
                _ => continue
            }
        };
    }

    pub fn update_no_input_components(&mut self) -> () {
        for i in 0..self.positions.len() {
            match (self.positions[i], self.inputs[i]) {
                (Some(position), None) => match self.map.get_mut(position.y_pos as usize, position.x_pos as usize) {
                    Some(tile) => {
                        tile.current_ids.push(i);
                    },
                    None => println!("Update no input components failed on tile {:?}", position)
                },
                _ => continue
            }
        };
    }


    pub fn add_position(&mut self, display: Display, position: Position, debug: Debug) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug.clone());
                self.occupancies[id] = Occupancy {position: true, display: true, input:false, debug: true, physics:false};
                true
            },
            None => false
        }
    }

    pub fn add_position_input(&mut self, display: Display, position: Position, input: Input, debug: Debug) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug);
                self.inputs[id] = Some(input);
                self.occupancies[id] = Occupancy {position: true, display: true, input:true, debug: true, physics: false};
                true
            },
            None => false
        }
    }

    pub fn add_physics_position(&mut self, display: Display,physics: Physics, position: Position, debug: Debug) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug);
                self.physics[id] = Some(physics);
                self.occupancies[id] = Occupancy {position: true, display: true, input:false, debug: true, physics:true};
                true
            },
            None => false
        }
    }

    pub fn add_physics_position_input(&mut self, display: Display,physics: Physics, position: Position, input: Input, debug: Debug) -> bool {
        let available_id = self.available_id();
        match available_id {
            Some(id) => {
                self.positions[id] = Some(position);
                self.display[id] = Some(display);
                self.debug[id] = Some(debug);
                self.inputs[id] = Some(input);
                self.physics[id] = Some(physics);
                self.occupancies[id] = Occupancy {position: true, display: true, input:true, debug: true, physics:true};
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
                self.physics[id] = None;
                self.current_moves[id] = None;
                self.occupancies[id] = Occupancy::new();
                true
            }

        }
    }



}

