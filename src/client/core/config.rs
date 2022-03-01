use dirs::home_dir;
use std::fs;

fn get_home_dir() -> String {
    match home_dir() {
        Some(home) => match home.to_str() {
            Some(str) => return str.to_string(),
            None => panic!("home_dir is not converted"),
        },
        None => panic!("home_dir is none"),
    };
}

fn make_config_path(path: &str) -> String {
    return format!("{}/.switch_bot/config/{}", get_home_dir(), path);
}

fn trim(str: String) -> String {
    let mut new_str = str.clone();
    new_str.retain(|c| c != '\n');
    return new_str;
}

pub fn get_token() -> String {
    let result = fs::read_to_string(make_config_path("token"));

    match result {
        Err(why) => {
            panic!("{:#?}", why);
        }
        Ok(token) => return trim(token),
    }
}

pub fn get_device_id(device_name: &str) -> String {
    let result = fs::read_to_string(format!("{}/{}", make_config_path("devices"), device_name));

    match result {
        Err(why) => {
            panic!("{:#?}", why);
        }
        Ok(token) => {
            return trim(token);
        }
    }
}
