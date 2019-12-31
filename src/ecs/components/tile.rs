#[derive(Debug, Clone)]
pub struct Tile {
    pub current_ids: Vec<u64>,
    pub new_ids: Vec<u64>
}

impl Tile {
    pub fn new(initial_capacity: usize)->Tile {
        Tile {
            current_ids: Vec::with_capacity(initial_capacity),
            new_ids: Vec::with_capacity(initial_capacity)
        }
    }

    pub fn clear_current(&mut self)->(){
        self.current_ids.clear();
    }

    pub fn clear_new(&mut self)->(){
        self.new_ids.clear()
    }
}