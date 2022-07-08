use std::env;
mod client;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        "devices" => println!("{:#?}", client::get_devices()),

        "room_light_on" => client::post_custom_command("room_light", "全灯"),
        "room_light_night" => client::post_custom_command("room_light", "お好みの明るさ"),
        "room_light_off" => client::post_turn_off_command("room_light"),

        "desk_light_on" => client::post_turn_on_command("desk_light"),
        "desk_light_off" => client::post_turn_off_command("desk_light"),

        "ac_on" => client::post_turn_on_command("ac"),
        "ac_off" => client::post_turn_off_command("ac"),

        "fan_volume" => client::post_custom_command("fan", "風量"),

        _ => println!("unknown command!"),
    }
}
