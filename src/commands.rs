
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
    },
    Command {
        name: "cd",
        description: "change directory",
        usage: "cd [target-path]"
    },
    Command {
        name: "pwd",
        description: "print working directory",
        usage: "pwd"
    },
    Command {
        name: "ls",
        description: "list all files and folders in current working directory",
        usage: "ls -f"
    }
];

pub fn help(cmd: Vec<&str>) {

    if cmd.len() == 1 {
        set_style(&Style::HEADING);
        println!("available commands:");

        for command in COMMANDS {
            set_style(&Style::CMD_NAME);
            print!("{:<12}", command.name);
            set_style(&Style::NORMAL);
            println!("{}", command.description);
        }
    } else if cmd.len() == 2 {
        if let Some(arg) = cmd.get(1) {
            if let Some(command) = COMMANDS.iter().find(|c| c.name == *arg) {

                set_style(&Style::HEADING);
                print!("{:<14}", "help: ");
                set_style(&Style::CMD_NAME);
                println!("{}", command.name);

                set_style(&Style::HEADING);
                print!("{:<14}", "description: ");
                set_style(&Style::NORMAL);
                println!("{}", command.description);

                set_style(&Style::HEADING);
                print!("{:<14}", "usage: ");
                set_style(&Style::CMD_NAME);
                println!("{}", command.usage);

            } else {
                set_style(&Style::ERROR);
                println!("error: command '{}' not found", cmd.get(1));
            }
    } else {
        set_style(&Style::ERROR);
        println!("wrong usage: expected 1 or 2 arguments, got {}", cmd.len());
    }
}
