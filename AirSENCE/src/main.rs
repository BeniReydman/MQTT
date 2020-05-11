extern crate rmp_serde as rmps;

use std::str;

use rumqtt::{MqttClient, MqttOptions, QoS};

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
//use rmps::decode::{self, Error};

use toml_reader::main::parse as toml_parse;
use toml_reader::structs::config::*;

// const SERVER_IP: &str = "142.112.23.87";
// const SERVER_PORT: i32 = 1883;
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

fn main() {
    // Serialize toml config to a msgpack
    let mut buf = Vec::new();
    let config: Config = toml_parse("C:\\Users\\Beni Reydman\\Documents\\Work\\Rust Code\\server\\toml_reader\\files\\config.toml".to_string());
    config.serialize(&mut Serializer::new(&mut buf)).unwrap();

    // Create server for testing
    let mqtt_options = MqttOptions::new("Pgm2", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    mqtt_client.subscribe("Test", QoS::AtLeastOnce).unwrap();
    //let payload = format!(buf);
    mqtt_client.publish("Test", QoS::AtLeastOnce, false, buf.clone()).unwrap();

    // let notification = notifications.recv(); // -> To recv only 1
    // println!("{:?}", notification);
    for notification in notifications {
         println!("{:?}", notification)
    }
    
    let mut de = Deserializer::new(&buf[..]);
    let res: Config = Deserialize::deserialize(&mut de).unwrap();
    println!("Deserialized to: \n{:?}", res);
}
