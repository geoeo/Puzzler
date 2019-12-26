use super::model::{level::Level};
use super::model::commands::InputCommand;

use crossterm::{queue, style, Result, cursor};
use std::io::Write;
use crate::ecs::components::display::{Display, ERROR_ICON};
use crate::ecs::components::position::Position;

pub fn generate_boundaries(world: &Level) -> (String, String) {
    let boundary_width = world.width+2;
    let full_boundary = vec!['#'; boundary_width].iter().collect::<String>();
    let mut partial_boundary_vec = vec![' '; boundary_width];
    partial_boundary_vec[0] = '#';
    partial_boundary_vec[(boundary_width-1)] = '#';
    let partial_boundary = partial_boundary_vec.iter().collect::<String>();


    let mut world_string = String::new();
    world_string.push_str(&full_boundary);
    for _ in 0..world.height {
        world_string.push_str(&partial_boundary);
    }
    world_string.push_str(&full_boundary);


    (full_boundary, partial_boundary)
}


pub fn draw_boundary<W>(output: &mut W, level_height: usize, full_boundary: &str, partial_boundary: &str) -> Result<()> where W: Write {

    queue!(
            output,
            style::Print(full_boundary),
            cursor::MoveToNextLine(1)
        )?;

    for _ in 0..level_height {
        queue!(
            output,
            style::Print(partial_boundary),
            cursor::MoveToNextLine(1)
        )?;
    }

    queue!(
            output,
            style::Print(full_boundary),
            cursor::MoveToNextLine(1)
        )
}

pub fn draw_world<W>(output: &mut W, level: &Level) ->  Result<()> where W: Write {

    let x_offset: u16 = 1;
    let y_offset: u16 = 1;

    for x in 0..level.width {
        for y in 0..level.height {
            let x_term_pos = x as u16 +x_offset;
            let y_term_pos = y as u16 +y_offset;
            let display = level.map.get(y as usize,x as usize);
            match display {
                Some(display) =>
                    queue!(
                    output,
                    cursor::MoveTo(x_term_pos, y_term_pos),
                    style::Print(display.icon),
                    ),
                None => queue!(
                    output,
                    cursor::MoveTo(x_term_pos, y_term_pos),
                    style::Print(ERROR_ICON),
                    )
            }?

        }
    }

    queue!(
            output,
            cursor::MoveTo(0,level.height as u16 + 2)
    )
}

pub fn draw_display<W>(output: &mut W, display: &Display, position: &Position)->Result<()> where W: Write {
    queue!(output,cursor::MoveTo(position.x_pos, position.y_pos), style::Print(format!("{:}",display.icon)))
}

pub fn draw_game_command<W>(output: &mut W, command: &InputCommand) -> Result<()> where W: Write {
    queue!(output, style::Print(format!("{:?}",command.key_event)), cursor::MoveRight(1))
}



