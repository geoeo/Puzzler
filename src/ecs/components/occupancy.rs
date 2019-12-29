#[derive(Debug,Copy,Clone)]
pub struct Occupancy {
    pub display: bool,
    pub position: bool,
    pub input:bool,
    pub debug: bool
}

impl Occupancy {
    pub fn is_free(&self) -> bool {
        !(self.display || self.position || self.input || self.debug)
    }

    pub fn new() -> Occupancy {
        Occupancy {
            display: false,
            position: false,
            input: false,
            debug: false
        }
    }
}