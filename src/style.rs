// this file contains the text styling

use std::fmt;

// i want a function to make text formatting easy

pub enum Color {
    Red, Green, Yellow, Blue, White, Default
}

pub enum Weight {
    Normal, Bold, Dim
}

pub enum Decoration {
    Normal, Italic, Underline
}

pub fn set_style(color: Color, weight: Weight, decoration: Decoration) {
    let color_code = match color {
        Color::Red      => 31,
        Color::Green    => 32,
        Color::Yellow   => 33,
        Color::Blue     => 34,
        Color::White    => 39
    };

    let weight_code = match weight {
        Weight::Normal  => 22,
        Weight::Bold    => 1,
        Weight::Dim     => 2
    };

    let decoration_code = match decoration {
        Decoration::Normal      => 0,
        Decoration::Underline   => 4,
        Decoration::Italic      => 3
    };

    print!("\x1b[{};{};{}m", color_code, weight_code, decoration_code);
}

pub fn clear_style() {
    print!("\x1b[0m");
}

pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

pub fn test() {
    // should iterate through all combinations to check them out once
}
