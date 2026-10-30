// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

use std::io::{self, Write};

mod texts;
mod style;
mod typewriter;
use style::{Style, StyledText};

use std::sync::LazyLock;
static PROMPT: LazyLock<StyledText> = LazyLock::new(|| ">> ".green().bold());

fn main() {
//    typewriter::typewriter(&texts::INTRO.green().italic(), 100);

    loop {

        print!("{}", *PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("failed");
        
        if input.trim() == "exit" { 
            break; 
        } else if input.trim() == "clear" {
            style::clear_screen();
        } else {
            style::clear_styling();
            print!("{}{}", "you entered: ".green().italic(), input.red().bold());
        }
    }
}
