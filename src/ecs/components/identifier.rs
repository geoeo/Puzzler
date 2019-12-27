#[derive(Debug, Clone)]
pub struct Identifier{
    pub display: char,
    pub name: String //TODO: maybe move this somewhere else as it induces unncecessary clones
}