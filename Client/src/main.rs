extern crate rmp_serde as rmps;

use std::str;
use std::sync::Arc;

use rumqtt::{MqttClient, MqttOptions, QoS};
use rumqtt::client::Notification;


use serde::{Deserialize};
use rmps::{Deserializer};

use toml_reader::structs::config::*;

// const SERVER_IP: &str = "142.112.23.87";
// const SERVER_PORT: i32 = 1883;
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 1883;

fn main() {
    // Get server settings
    let mqtt_options = MqttOptions::new("Pgm1", SERVER_IP, SERVER_PORT);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
      
    // Subscribe to server to receive publishes
    mqtt_client.subscribe("Test", QoS::AtLeastOnce).unwrap();

    // Parse notifications
    for notification in notifications {
        match notification {
            Notification::Publish(publish) =>  print_notification(Arc::try_unwrap(publish.payload).unwrap()),
            _ => println!("Received something that's not a publish! {:?}", notification)
        }
    }
}

pub fn print_notification(payload: Vec<u8>) {
    let mut de = Deserializer::new(&payload[..]);
    let res: RawData = Deserialize::deserialize(&mut de).unwrap();
    println!("Deserialized to: \n{:?}", res);
}
