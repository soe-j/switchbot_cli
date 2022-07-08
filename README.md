# my_room

SwitchBot CLI

## example

```
~/.switch_bot/config
├── devices
│   ├── desk_light ... device_id
│   └── room_light ... device_id
└── token ... get from SwitchBot app!
```

```bash
# show your devices
$ my_room devices

# setup device ID
$ echo 'desk-light-device-id' > devices/desk_light

# custom main.rs
"desk_light_on" => client::post_turn_on_command("desk_light"),

# build
$ cargo build

# light on!
$ my_room desk_light_on

```

Try to customize `src/main.rs`!
