use puzzler::run;

use puzzler::ecs::entities::level::Level;

fn main() {
    let mut stdout = std::io::stdout();

    let mut level = Level::new(10, 10);

    match run(&mut stdout, &mut level) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}