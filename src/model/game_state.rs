#[repr(u8)]
#[derive(Copy,Clone,PartialEq)]
pub enum GameState {
    Running,
    Pause,
    WorldMenu,
    LevelMenu,
    QuitLevel,
    QuitGame,
}

