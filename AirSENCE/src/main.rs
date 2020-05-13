extern crate rmp_serde as rmps;
pub mod custom_errors;

use std::str;

use rumqtt::{MqttClient, MqttOptions, QoS};

use serde::{Serialize};
use rmps::{Serializer};

use toml_reader::main::parse as toml_parse;
use toml_reader::structs::config::*;

use log::{info, warn};
use custom_errors::MyError;

// const SERVER_IP: &str = "142.112.23.87";
// const SERVER_PORT: i32 = 1883;
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

fn main() {
    // Serialize toml config to a msgpack
    let buf = match serialize_toml("C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\toml_reader\\files\\config.toml") {
        Ok(buf) => buf,
        _ => return
    };

    // Create server for testing
    info!("Initiating MQTT Client.");
    let mqtt_options = MqttOptions::new("Pgm2", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    
    mqtt_client.subscribe("Test", QoS::AtLeastOnce).unwrap();
    info!("Listening for requests.");
    mqtt_client.publish("Test", QoS::AtLeastOnce, false, buf.clone()).unwrap();

    // let notification = notifications.recv(); // -> To recv only 1
    // println!("{:?}", notification);
    for notification in notifications {
        info!("Received request.");
        println!("{:?}", notification);
    }
    
    // let mut de = Deserializer::new(&buf[..]);
    // let res: Config = Deserialize::deserialize(&mut de).unwrap();
    // println!("Deserialized to: \n{:?}", res);
}

/***
* Function serialize_toml:
*
* Purpose:
* serialize read toml file given path
***/
fn serialize_toml(path: &str) ->Result<Vec<u8>, MyError> {
    // Get config struct from toml
    let config: Config = toml_parse(path.to_string());  
    match config {
        config if config == Config::default() => {
            warn!("Empty Config, Ignoring request.");
            return Err(MyError::TomlParseError)
        }
        _ => info!("Successfully parsed toml.")
    }

    // Serialize config into buf
    let mut buf = Vec::new();
    match config.serialize(&mut Serializer::new(&mut buf)) {
        Ok(_) => return Ok(buf),
        Err(e) => {
            warn!("Error serializing: {:?}", e);
            return Err(MyError::SerializeError)
        }
    }
}