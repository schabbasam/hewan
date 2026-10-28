// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

mod texts;
mod style;
mod typewriter;
use style::Style;

fn main() {
    typewriter::typewriter(texts::INTRO.green().italic(), 80);
}
