extern crate rmp_serde as rmps;
extern crate rand;
pub mod custom_errors;

use std::str;
use std::io::stdin;

use rumqtt::{MqttClient, MqttOptions, QoS};

use serde::{Serialize, Deserialize};
use rmps::{Serializer, Deserializer};

use toml_reader::main::parse as toml_parse;
use toml_reader::structs::config::*;

use log::{info, warn};
use custom_errors::MyError;

use rand::Rng;

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

fn main() {
    // read
    // serialize
    // publish

    testPublish();


    // let result = publish("C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\toml_reader\\files\\config.toml");
    // match result {
    //     Ok(_) => println!("Success!"),
    //     Err(_) => println!("Unsuccessful.")
    // }
}

fn testPublish() ->Result<(), MyError> {
    // Initialize MQTT server settings
    info!("Initiating MQTT Client.");
    let mqtt_options = MqttOptions::new("Pgm2", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    
    let mut buf = match serialize_struct(generate_raw_data()) {
        Ok(buf) => buf,
        Err(error) => return Err(error)
    };

    println!("{:?}", buf);

    println!("write anything to publish, hit enter to quit!");
    let mut input = String::new();
    loop {
        stdin().read_line(&mut input)
        .expect("Failed to read line");

        if input.trim() == "quit".to_string() {
            println!("Quitting!");
            break;
        }

        // Clear string
        input = String::new();

        mqtt_client.publish("Test", QoS::AtLeastOnce, false, buf.clone()).unwrap();

        // Create new struct to publish
        buf = match serialize_struct(generate_raw_data()) {
            Ok(buf) => buf,
            Err(error) => return Err(error)
        };
        println!("{:?}", buf);
    }

    return Ok(())
}

pub fn publish(path: &str) ->Result<(), MyError> {
    // Serialize toml config to a msgpack
    let buf = match serialize_toml(path) {
        Ok(buf) => buf,
        Err(error) => return Err(error)
    };

    // Initialize MQTT server settings
    info!("Initiating MQTT Client.");
    let mqtt_options = MqttOptions::new("Pgm2", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    
    // Publish request
    //mqtt_client.subscribe("Test", QoS::AtMostOnce).unwrap();
    //info!("Listening for requests.");
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

    // Serialize config 
    return serialize_struct(config);
}

/***
* Function serialize_struct:
*
* Purpose:
* serialize structs into message packs
***/
fn serialize_struct<T>(data: T) -> Result<Vec<u8>, MyError> where T: Serialize, {
    let mut buf = Vec::new();
    let mut msg_pack = Serializer::new(&mut buf);
    match data.serialize(&mut msg_pack) {
        Ok(_) => return Ok(buf),
        Err(e) => {
            warn!("Error serializing: {:?}", e);
            return Err(MyError::SerializeError)
        }
    }
}

fn generate_raw_data() -> RawData {
    let raw_data = RawData {
        AQHI:		generate_i32(),
        AQI:		generate_i32(),
        CO:			generate_f32(),
        CO2:		generate_f32(),
        NO:			generate_f32(),
        NO2:		generate_f32(),
        O3:			generate_f32(),
        PM1:		generate_f32(),
        PM2_5:		generate_f32(),
        PM10:		generate_f32(),
        SO2:		generate_f32(),
        T:			generate_f32(),
        RH:			generate_f32(),
        NOISE:		generate_f32(),
        TimeStamp:	Some("".to_string())
    };

    return raw_data;
}

fn generate_i32() -> Option<i32> {
    let mut rng = rand::thread_rng();
    let num: i32 = rng.gen_range(0,10);
    if num >= 8 {
        return None;
    }
    return Some(num);
}

fn generate_f32() -> Option<f32> {
    let mut rng = rand::thread_rng();
    let num: f32 = rng.gen_range(0.0,10.0);
    if num >= 8.0 {
        return None;
    }
    return Some(num);
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
        let bad_path = "C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\Broker\\AirSENCE\\toml_reader\\files\\config.toml";
        assert!(serialize_toml(bad_path).is_err());
    }
}