
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
    println!("available commands:");
    for command in COMMANDS {
        println!("{:<12} {}", command.name, command.description);
    }
}
