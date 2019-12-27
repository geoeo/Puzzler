use crate::model::game_state::GameState;
use crate::model::level::Level;
use crate::view::{generate_boundaries, draw_boundary, input_game_command, draw_world};
use crate::ecs::systems::input::Input;
use crate::ecs::components::keyboard_input::KeyboardInput;

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
use crate::ecs::components::position::Position;
use crate::ecs::systems::movable::Movable;

pub mod view;
pub mod model;
pub mod ecs;

pub fn run<W>(output: &mut W, level: &mut Level) -> Result<()> where W: Write{

    // encapsulate this better
    let (full,partial) = generate_boundaries(&level);
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
                draw_boundary(output, level.height, &full, &partial)?;
                draw_world(output, level)?;
                let read = read()?;
                let (game_state_new, input_command_new) = KeyboardInput::parse_input_event(&read);
                game_state = game_state_new;
                let move_command = KeyboardInput::process_input(&input_command_new);
                Position::apply_move_on_all(&mut level.positions, &move_command);

                input_game_command(output, &input_command_new)?;
            }

            Ok(false) => {
                //draw_world(output, world, &full, &partial)?;
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