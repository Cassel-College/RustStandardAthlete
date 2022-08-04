
extern crate ansi_term;
use crate::log::level::level::level::get_level_code;
use crate::log::log_title::log_title::level_code2title;
use crate::log::log_title::log_color::get_title_with_color;

pub fn log4rs(log: String, level: String) {
    let level_code: i8 = get_level_code(level);
    let title: String = level_code2title(level_code, "EN".to_string());
    println!("{}: {}", get_title_with_color(title, level_code), log);
}