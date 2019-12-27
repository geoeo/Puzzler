#[derive(Debug,Copy,Clone)]
pub struct Occupancy {
    display: bool,
    position: bool,
    keyboard_input:bool,
    identifier: bool
}

impl Occupancy {
    pub fn is_free(&self) -> bool {
        !(self.display || self.position || self.keyboard_input || self.identifier)
    }

    pub fn new() -> Occupancy {
        Occupancy {
            display: false,
            position: false,
            keyboard_input: false,
            identifier: false
        }
    }
}