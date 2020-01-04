use puzzler::run;

use puzzler::ecs::entities::level::Level;
use puzzler::ecs::components::position::Position;
use puzzler::ecs::systems::commands::{simple_command, no_command, simple_move};
use puzzler::ecs::components::display::Display;
use puzzler::ecs::components::debug_information::Debug;
use puzzler::ecs::components::input::Input;
use puzzler::ecs::components::physics::Physics;

fn main() {
    let mut stdout = std::io::stdout();

    let mut level = Level::new(10, 10);

    //Player
    let pos = Position {
        x_pos: 1,
        y_pos: 4
    };
    let disp = Display {
        icon: 'c'
    };
    let debug = Debug {
        name: String::from("player")
    };
    let simple_input = Input {
        keyboard_delegate: simple_command
    };

    // Obstacle
    let obs_pos = Position {
        x_pos: 3,
        y_pos: 3
    };
    let obs_disp = Display {
        icon: 'W'
    };
    let obs_debug = Debug {
        name: String::from("obstacle")
    };

    let obs_simple_input = Input {
        keyboard_delegate: no_command
    };

    let obs_physics = Physics {
        repulsion_factor: -1,
        move_delegate: simple_move
    };


    let player_succ = level.add_position_input(disp,pos,simple_input,debug);
    assert!(player_succ);

    let obs_succ = level.add_physics_position(obs_disp, obs_physics,obs_pos, obs_debug);
    assert!(obs_succ);

    match run(&mut stdout, &mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}