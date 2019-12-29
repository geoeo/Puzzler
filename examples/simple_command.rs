use puzzler::run;

use puzzler::model::level::Level;
use puzzler::ecs::components::position::Position;
use puzzler::ecs::systems::commands::simple_command;
use puzzler::ecs::components::display::Display;
use puzzler::ecs::components::debug_information::DebugInformation;
use puzzler::ecs::components::input::Input;

fn main() {
    let mut stdout = std::io::stdout();

    let mut level = Level::new(10, 10);
    let pos = Position {
        x_pos: 1,
        y_pos: 4
    };
    let disp = Display {
        icon: 'c'
    };
    let debug = DebugInformation {
        name: String::from("debug_name")
    };
    let simple_input = Input {
        keyboard_delegate: simple_command
    };

    let succ = level.add_position_input(disp,pos,simple_input,debug);
    assert!(succ);

    match run(&mut stdout, &mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}