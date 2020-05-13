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
    let result = publish("C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\toml_reader\\files\\config.toml");
    match result {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Unsuccessful.")
    }
}

pub fn publish(path: &str) ->Result<(), MyError>{
    // Serialize toml config to a msgpack
    let buf = match serialize_toml(path) {
        Ok(buf) => buf,
        Err(error) => return Err(error)
    };

    // Initialize MQTT server settings
    info!("Initiating MQTT Client.");
    let mqtt_options = MqttOptions::new("Pgm2", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    
    // Subscribe and then publish request
    mqtt_client.subscribe("Test", QoS::AtLeastOnce).unwrap();
    info!("Listening for requests.");
    mqtt_client.publish("Test", QoS::AtLeastOnce, false, buf.clone()).unwrap();

    // Receive single return value
    let notification = notifications.recv(); 
    println!("{:?}", notification);

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_path() {
        let real_path = "C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\toml_reader\\files\\config.toml";
        assert!(serialize_toml(real_path).is_ok());
    }

    #[test]
    fn fake_path() {
        let fake_path = "blah blah";
        assert!(serialize_toml(fake_path).is_err());
    }

    #[test]
    fn bad_file() {
        let fake_path = "C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\Broker\\AirSENCE\\toml_reader\\files\\config.toml";
        assert!(serialize_toml(fake_path).is_err());
    }
}