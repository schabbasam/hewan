
use crate::style::*;

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str
}

pub const COMMANDS: &[Command] = &[
    Command {
        name: "help",
        description: "show available commands and their usage",
        usage: "help [command]"
    },
    Command {
        name: "exit",
        description: "exit the program",
        usage: "exit"
    },
    Command {
        name: "clear",
        description: "clear the terminal screen",
        usage: "clear"
    }
];

pub fn help() {
    set_style(Color::Yellow, Weight::Bold, Decoration::Normal);
    println!("available commands:");
    for command in COMMANDS {
        set_style(Color::Green, Weight::Bold, Decoration::Italic);
        print!("{:<12}", command.name);
        set_style(Color::Blue, Weight::Normal, Decoration::Normal);
        println!("{}", command.description);
        
    }
}
