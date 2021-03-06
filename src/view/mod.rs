use crate::ecs::entities::level::Level;
use super::model::commands::InputCommand;

use crossterm::{queue, style, Result, cursor};
use std::io::Write;
use crate::ecs::components::{position::Position,display::Display};

pub mod constants;

pub fn generate_boundaries(level: &Level) -> (String, String) {
    let boundary_width = level.width as usize+2;
    let full_boundary = vec!['#'; boundary_width as usize].iter().collect::<String>();
    let mut partial_boundary_vec = vec![' '; boundary_width as usize];
    partial_boundary_vec[0] = '#';
    partial_boundary_vec[(boundary_width-1)] = '#';
    let partial_boundary = partial_boundary_vec.iter().collect::<String>();


    let mut world_string = String::new();
    world_string.push_str(&full_boundary);
    for _ in 0..level.height {
        world_string.push_str(&partial_boundary);
    }
    world_string.push_str(&full_boundary);


    (full_boundary, partial_boundary)
}


pub fn draw_boundary<W>(output: &mut W, level_height: u16, full_boundary: &str, partial_boundary: &str) -> Result<()> where W: Write {

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

//TODO: maybe map should only hold (id,char)
pub fn draw_world<W>(output: &mut W, level: &Level) ->  Result<()> where W: Write {

    let x_offset: u16 = 1;
    let y_offset: u16 = 1;

    for x in 0..level.width {
        for y in 0..level.height {
            let x_term_pos = x as u16 +x_offset;
            let y_term_pos = y as u16 +y_offset;
            let element = level.map.get(y as usize,x as usize);
            match element {
                Some(tile) => {
                    match tile.current_ids.len() {
                        0 =>  queue!(
                                    output,
                                    cursor::MoveTo(x_term_pos, y_term_pos),
                                    style::Print(constants::EMPTY_ICON),
                                    ),
                        1 => {
                            let display = level.display[tile.current_ids[0] as usize].unwrap();
                            queue!(
                                    output,
                                    cursor::MoveTo(x_term_pos, y_term_pos),
                                    style::Print(display.icon),
                                    )

                        },
                        _ =>  queue!(
                                    output,
                                    cursor::MoveTo(x_term_pos, y_term_pos),
                                    style::Print(constants::MULTIPLE_ICON),
                                    )

                    }
                }

                None => queue!(
                    output,
                    cursor::MoveTo(x_term_pos, y_term_pos),
                    style::Print(constants::ERROR_ICON),
                    )
            }?

        }
    }

    queue!(
            output,
            cursor::MoveTo(0,level.height as u16 + 2)
    )
}

pub fn draw_display<W>(output: &mut W, display: &Display, position: &Position) ->Result<()> where W: Write {
    queue!(output,cursor::MoveTo(position.x_pos, position.y_pos), style::Print(format!("{:}",display.icon)))
}

pub fn draw_input_command<W>(output: &mut W, command: &InputCommand) -> Result<()> where W: Write {
    queue!(output, style::Print(format!("{:?}",command.key_event)), cursor::MoveRight(1))
}



