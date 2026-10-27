// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

// i want a function to make text
// formatting easy

// TODO chaining these functions and stacking
// the effect unfortunatly doesn't work as of know

trait Style {
    fn red(&self) -> String;
    fn bold(&self) -> String;
}

impl Style for str {
    fn red(&self) -> String {
        format!("\x1b[31m{self}\x1b[0m")
    }

    fn bold(&self) -> String {
        format!("\x1b[1m{self}\x1b[0m")
    }
}

fn main() {
    println!("{}", "red text aaaaaaaaa".red());
    println!("{}", "bold text aaaaaaaaa".bold());
    println!("{}", "red and bold text aaaaaaaa".red().bold());
}
