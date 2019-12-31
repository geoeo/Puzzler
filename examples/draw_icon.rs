use puzzler::run;

use puzzler::model::level::Level;
use puzzler::ecs::components::position::Position;
use puzzler::ecs::components::display::Display;
use puzzler::ecs::components::debug_information::Debug;

fn main() {
    let mut stdout = std::io::stdout();

    let mut level = Level::new(10, 10);
    let pos = Position {
        x_pos: 1,
        y_pos: 4
    };
    let id = Display {
        icon: 'c'
    };
    let debug = Debug {
        name: String::from("debug_name")
    };

    level.positions.push(Some(pos));
    level.display.push(Some(id));
    level.debug.push(Some(debug));

    match run(&mut stdout, &mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}