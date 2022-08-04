use crate::log::log_title::language::en::log_info_en;
use crate::log::log_title::language::zh::log_info_zh;


pub fn level_code2title(level_code: i8, language: String) -> String {
    
    let title: String;
    if level_code == 1 {
        if language.to_lowercase().as_str() == "en" {
            title = log_info_en::get_info_title();
        } else {
            title = log_info_zh::get_info_title();
        }
    } else if level_code == 2 {
        if language.to_lowercase().as_str() == "en" {
            title = log_info_en::get_debug_title();
        } else {
            title = log_info_zh::get_debug_title();
        }
    } else if level_code == 3 {
        if language.to_lowercase().as_str() == "en" {
            title = log_info_en::get_error_title();
        } else {
            title = log_info_zh::get_error_title();
        }
    } else {
        if language.to_lowercase().as_str() == "en" {
            title = log_info_en::get_info_title();
        } else {
            title = log_info_zh::get_info_title();
        }
    }
    return title;
}