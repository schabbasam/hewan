// this is supposed to be a game where you
// have to connect the dots (metaphorically)
// and solve riddles on an old computer terminal
// in an abandonded antique labratory

use std::fmt;

// i want a function to make text formatting easy

// this is a data sctructure with two fields
struct StyledText {
    text: String,
    // Vec is a dynamic array, basically a mutable list
    // the thing in the pointy brackets defines what type of
    // elements the Vec contains. 'static is the lifetime,
    // meaning they have to stay for the whole program
    codes: Vec<&'static str>
}

// a trait describes what abilities an objects has
// that means every type that implements this trait has
// to define these two methods
trait Style {
    fn red(&self) -> StyledText;
    fn green(&self) -> StyledText;
}

impl Style for str {
    fn red(&self) -> StyledText {
        StyledText {
            // to_string simply converts self into a String
            text: self.to_string(),
            // a makro is different from a normal function
            // it turns into source code while compiling
            // and the parameters can differ in number or types
            // in this case, we create a Vec with a single element
            codes: vec!["31"]
            // why put a number in quotation marks?
            // we actually need a string for the formatting code
        }
    }

    fn green(&self) -> StyledText {
        StyledText {
            text: self.to_string(),
            codes: vec!["32"]
        }
    }
}

impl StyledText {
    // in these functions, we need to edit the object
    // thats why we need to use mut

    // -> Self means the function returns the same type
    // as it implements, which allows method chaining
    fn bold(mut self) -> Self {
        // push adds an element at the end of codes
        self.codes.push("1");
        self // returns self
    }

    fn italic(mut self) -> Self {
        self.codes.push("3");
        self
    }

    fn underline(mut self) -> Self {
        self.codes.push("4");
        self
    }
}

// implementing the Display trait from std::fmt
impl fmt::Display for StyledText {
    // f is the name of the parameter
    // the type is a "mutable reference to a Formatter object"
    // <'_> this is a lifetime indicator, _ means let rust decide
    // the return value is a fmt::Result type
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( // i suppose this macro creates a fmt::Result
            f,
            "\x1b[{codes}m{text}\x1b[0m",
            // the line below joins all elements of the Vec seperated by ;
            codes=self.codes.join(";"),
            text=self.text
        )
    }
}

fn main() {
    println!("{}", "red text aaaaaaaaa".red());
    println!("{}", "bold text aaaaaaaaa".green().bold());
    println!("{}", "red and bold text aaaaaaaa".red().bold());
}
