use puzzler::run;

use puzzler::model::level::Level;

fn main() {
    let mut stdout = std::io::stdout();

    let world = Level::new(10,10);

    match run(&mut stdout, &world) {
        Err(e) => panic!(e),
        Ok(_) => ()
    }
}