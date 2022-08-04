

mod soft_name {
    pub fn get_default_soft_name() -> String {
        return String::from("Athlete");
    }
    
    pub fn get_chainese_soft_name() -> String {
        return String::from("健将");
    }
    
    pub fn get_english_soft_name() -> String {
        return String::from("Athlete");
    }

    pub fn get_soft_name(language: String) -> String {
        let soft_name: String;
        if language.as_str() == "ZH" {
            soft_name = get_chainese_soft_name();
        } else if language.as_str() == "EN" {
            soft_name = get_english_soft_name();
        } else {
            soft_name = get_default_soft_name();
        }
        return soft_name;
    }
}

mod soft_version {
    pub fn get_version_num() -> (i8, i8, i8) {
        let big_version: i8 = 0;
        let small_version: i8 = 1;
        let sub_version: i8 = 1;
        return (big_version, small_version, sub_version);
    }
    
    pub fn get_version_str() -> String {
        let version = get_version_num();
        let version_info: String = version.0.to_string() + "." + version.1.to_string().as_str() + "." + version.2.to_string().as_str();
        return version_info;
    }
    
    pub fn get_default_version_word() -> String {
        return String::from("Verion: ");
    }
    
    pub fn get_chainese_version_word() -> String {
        return String::from("版本: ");
    }
    
    pub fn get_english_version_word() -> String {
        return String::from("Verion: ");
    }
    
    pub fn get_version_info(language: String) -> String {
        let version_info: String;
        if language.as_str() == "ZH" {
            version_info = get_chainese_version_word() + get_version_str().as_str();
        } else if language.as_str() == "EN" {
            version_info = get_english_version_word() + get_version_str().as_str();
        } else {
            version_info = get_default_version_word() + get_version_str().as_str();
        }
        return version_info.to_string();
    }
}

use soft_name as name;
use soft_version as version;

pub fn get_version() -> String {
    let language: String = "EN".to_string();
    return name::get_soft_name(language.clone()) + " " + version::get_version_info(language.clone()).as_str();
}