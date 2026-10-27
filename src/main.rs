// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

mod style;
use style::Style;

fn main() {
    println!("{}", "red text aaaaaaaaa".red());
    println!("{}", "bold text aaaaaaaaa".green().bold());
    println!("{}", "red and bold text aaaaaaaa".red().bold());
}
