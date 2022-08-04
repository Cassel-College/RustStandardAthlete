extern crate ansi_term;
use ansi_term::Colour;


pub fn get_title_with_color(title: String, level_code: i8) -> ansi_term::ANSIString<'static>{

    match level_code {
        1 => Colour::Yellow.paint(title),
        2 => Colour::Blue.paint(title),
        3 => Colour::Red.paint(title),
        _ => Colour::Red.paint(title),
    }
}