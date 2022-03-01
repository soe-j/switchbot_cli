mod core;
mod request;
mod response;

pub fn get_devices() -> response::devices::DevicesResponse {
    return core::get::<response::devices::DevicesResponse>("/devices");
}

pub fn get_device_id(device_name: &str) -> String {
    return core::config::get_device_id(device_name);
}

pub fn post_custom_command(device_name: &str, command: &str) {
    let body = request::command::CommandRequest {
        command: String::from(command),
        parameter: String::from("default"),
        command_type: String::from("customize"),
    };

    return core::post(
        &format!("/devices/{}/commands", get_device_id(device_name)),
        body,
    );
}

pub fn post_turn_on_command(device_name: &str) {
    let body = request::command::CommandRequest {
        command: String::from("turnOn"),
        parameter: String::from("default"),
        command_type: String::from("command"),
    };

    return core::post(
        &format!("/devices/{}/commands", get_device_id(device_name)),
        body,
    );
}

pub fn post_turn_off_command(device_name: &str) {
    let body = request::command::CommandRequest {
        command: String::from("turnOff"),
        parameter: String::from("default"),
        command_type: String::from("command"),
    };

    return core::post(
        &format!("/devices/{}/commands", get_device_id(device_name)),
        body,
    );
}
