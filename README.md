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
$ echo 'desk_light device id' > devices/desk_light

# light on!
$ my_room desk_light_on

```

Try to customize `src/main.rs`!
