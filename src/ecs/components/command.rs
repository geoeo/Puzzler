#[derive(Debug,Copy,Clone)]
pub enum CommandType {
    NoCommand,
    SimpleCommand
}


#[derive(Debug,Copy,Clone)]
pub struct SimpleCommand {}

#[derive(Debug,Copy,Clone)]
pub struct NoCommand {}