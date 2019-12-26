use crossterm::event::KeyEvent;

#[repr(u8)]
#[derive(Copy,Clone)]
pub enum MoveCommand {
    Left,
    Right,
    Up,
    Down,
    None
}

#[derive(Debug,Copy,Clone)]
pub struct InputCommand {
    pub valid: bool,
    pub key_event: KeyEvent
}