use crate::model::game_state::GameState;
use crate::model::level::Level;
use crate::view::{generate_boundaries, draw_boundary, draw_input_command, draw_world};

use std::{
    time::Duration,
};

use crossterm::{
    event::{poll, read},
    execute, queue, style,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
    Result,
    cursor,
};
use std::io::Write;
use crate::ecs::systems::{movement::apply_input_move, input::{parse_input_event, keyboard_input}};
use crate::ecs::systems::movement::apply_physics;

pub mod view;
pub mod model;
pub mod ecs;

pub fn run<W>(output: &mut W, level: &mut Level) -> Result<()> where W: Write{

    // encapsulate this better
    let (full,partial) = generate_boundaries(&level);
    level.update_map();
    let mut game_state = GameState::Running;

    execute!(output, terminal::EnterAlternateScreen)?;
    enable_raw_mode()?;
    while game_state == GameState::Running {
        queue!(
            output,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;


        match poll(Duration::from_millis(1000)) {
            Ok(true) => {
                level.clear();
                let read = read()?;
                let (game_state_new, input_command_new) = parse_input_event(&read);
                game_state = game_state_new;
                let move_command = keyboard_input::process_input(&input_command_new);
                apply_input_move(&mut level.positions, &level.inputs, &mut level.current_moves, level.width, level.height, &move_command, 1);
                apply_physics(&mut level.physics, &mut level.positions, &mut level.current_moves, level.width, level.height);

                level.update_map();

                draw_boundary(output, level.height, &full, &partial)?;
                draw_world(output, level)?;
                draw_input_command(output, &input_command_new)?;
            }

            Ok(false) => {
                draw_boundary(output, level.height, &full, &partial)?;
                draw_world(output, level)?;
                queue!(output, style::Print("no input detected"), cursor::MoveToNextLine(1))?;
            }

            Err(e) => {
                queue!(output, style::Print(e.to_string()), cursor::MoveToNextLine(1))?;
            }
        }

        output.flush()?;
    }

    execute!(
        output,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;
    disable_raw_mode()

}