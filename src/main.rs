mod version;
mod log;

use version::get_version;
use log::log::log4rs;

fn main() {

    log4rs("Start Running Athlete.".to_string(), "INFO".to_string());
    log4rs("get version infomarion.".to_string(), "DEBUG".to_string());
    let version_info: String = get_version();
    println!("{}", version_info);
}
