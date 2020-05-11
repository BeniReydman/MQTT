# MQTT
## An MQTT subscriber/publisher for AirSENCE
### Downloads
Ensure to have mosquitto.exe from [https://mosquitto.org/download/](https://mosquitto.org/download/)

### Usage
Add this to your `Cargo.toml`:

```toml
[dependencies]
AirSENCE = { git = "https://github.com/BeniReydman/MQTT/tree/master/AirSENCE" }
Client = { git = "https://github.com/BeniReydman/MQTT/tree/master/Client" }
```

### Run
1. Run mosquitto.exe
2. Run Client by typing `cargo run` in Client directory 
3. Run AirSENCE by typing `cargo run` in AirSENCE directory

### Output
Client should output a Config file 