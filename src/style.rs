// this file contains the text styling

use std::fmt;

// i want a function to make text formatting easy

// this is a data sctructure with two fields
// it needs to be public so we can use it in other files
pub struct StyledText {
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
pub trait Style {
    fn red(&self) -> StyledText;
    fn green(&self) -> StyledText;
}


// adding the Style trait to every str object
// the functions in this construct don't need the pub keyword
// because Style is already public
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


// surpresses the "methods never used" warning
#[allow(dead_code)]
impl StyledText {
    // in these functions, we need to edit the object
    // thats why we need to use mut

    // -> Self means the function returns the same type
    // as it implements, which allows method chaining
    pub fn bold(mut self) -> Self {
        // push adds an element at the end of codes
        self.codes.push("1");
        self // returns self
    }

    pub fn italic(mut self) -> Self {
        self.codes.push("3");
        self
    }

    pub fn underline(mut self) -> Self {
        self.codes.push("4");
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn codes(&self) -> &[&'static str] {
        &self.codes
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
            "\x1b[{codes}m{text}",
            // the line below joins all elements of the Vec seperated by ;
            codes=self.codes.join(";"),
            text=self.text
        )
    }
}

pub fn clear_styling() {
    print!("\x1b[0m");
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}
