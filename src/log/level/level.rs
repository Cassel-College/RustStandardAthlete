
pub mod level {
    pub fn get_level_code(level: String) -> i8 {
        let level_num: i8;
        match level.to_lowercase().as_str() {
            "info" => level_num = 1,
            "debug" => level_num = 2,
            "error" => level_num = 3,
            _ => level_num = 1,
        };
        return level_num;
    }
}
