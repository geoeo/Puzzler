pub const ERROR_ICON: char = 'E';

//TODO: also save id in entity lists
#[derive(Debug, Copy, Clone)]
pub struct Display {
    pub icon: char
}

impl Display {
    pub fn new(icon: char) -> Display {
        Display{icon}
    }
}