use puzzler::run;

use puzzler::model::level::Level;
use puzzler::ecs::components::position::Position;
use puzzler::ecs::systems::commands::no_command;
use puzzler::ecs::components::display::Display;

fn main() {
    let mut stdout = std::io::stdout();

    let mut level = Level::new(10, 10);
    let pos = Position {
        x_pos: 5,
        y_pos: 5,
        input_delegate: no_command
    };
    let id = Display {
        icon: 'c'
    };

    level.positions.push(Some(pos));
    level.identifiers.push(Some(id));

    match run(&mut stdout, &mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}